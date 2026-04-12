use super::*;
use xcell_parser::{TypeExpr, PrimitiveType as ParserPrimitiveType};
use crate::map::MapDescription;
use crate::optional::OptionalDescription;

impl XCellTyped {
    /// 使用 xcell-parser 解析类型表达式
    pub fn parse(input: &str, info: &TypeMetaInfo) -> Self {
        match xcell_parser::parse_type(input) {
            Ok(expr) => type_expr_to_typed(expr, info),
            Err(e) => {
                eprintln!("类型解析失败: '{}' 被解析为 Unknown 类型，错误: {}", input, e);
                Self::Unknown
            }
        }
    }
}

/// 从 TypeExpr 转换为 XCellTyped
fn type_expr_to_typed(expr: TypeExpr, info: &TypeMetaInfo) -> XCellTyped {
    match expr {
        TypeExpr::Primitive(p) => primitive_to_typed(p, info),
        TypeExpr::Reference { target } => ReferenceDescription::new(&target).into(),
        TypeExpr::List { element } => {
            let element_type = type_expr_to_typed(*element, info);
            ListDescription { element_type, ..Default::default() }.into()
        }
        TypeExpr::FixedArray { element, length } => {
            let element_type = type_expr_to_typed(*element, info);
            ListDescription {
                element_type,
                fixed_length: Some(length),
                ..Default::default()
            }.into()
        }
        TypeExpr::Vec { element } => {
            let element_type = type_expr_to_typed(*element, info);
            info.vector.clone().with_type(element_type).into()
        }
        TypeExpr::Generic { name, args } => {
            generic_to_typed(&name, args, info)
        }
        TypeExpr::Tuple(elements) => {
            if elements.is_empty() {
                info.string.clone().into()
            } else {
                let len = elements.len();
                match len {
                    2 => {
                        let mut iter = elements.into_iter();
                        let first = type_expr_to_typed(iter.next().unwrap(), info);
                        let second = type_expr_to_typed(iter.next().unwrap(), info);
                        if matches!(first, XCellTyped::Decimal(_)) && matches!(second, XCellTyped::Decimal(_)) {
                            ArrayDescription::new(ArrayKind::Vector2).into()
                        } else {
                            ListDescription {
                                element_type: first,
                                fixed_length: Some(2),
                                ..Default::default()
                            }.into()
                        }
                    }
                    3 => {
                        let mut all_decimal = true;
                        for elem in elements.iter() {
                            let t = type_expr_to_typed(elem.clone(), info);
                            if !matches!(t, XCellTyped::Decimal(_)) {
                                all_decimal = false;
                                break;
                            }
                        }
                        if all_decimal {
                            ArrayDescription::new(ArrayKind::Vector3).into()
                        } else {
                            let first = type_expr_to_typed(elements.into_iter().next().unwrap(), info);
                            ListDescription {
                                element_type: first,
                                fixed_length: Some(3),
                                ..Default::default()
                            }.into()
                        }
                    }
                    4 => {
                        let mut all_decimal = true;
                        for elem in elements.iter() {
                            let t = type_expr_to_typed(elem.clone(), info);
                            if !matches!(t, XCellTyped::Decimal(_)) {
                                all_decimal = false;
                                break;
                            }
                        }
                        if all_decimal {
                            ArrayDescription::new(ArrayKind::Vector4).into()
                        } else {
                            let first = type_expr_to_typed(elements.into_iter().next().unwrap(), info);
                            ListDescription {
                                element_type: first,
                                fixed_length: Some(4),
                                ..Default::default()
                            }.into()
                        }
                    }
                    _ => {
                        let first = type_expr_to_typed(elements.into_iter().next().unwrap(), info);
                        ListDescription {
                            element_type: first,
                            fixed_length: Some(len),
                            ..Default::default()
                        }.into()
                    }
                }
            }
        }
        TypeExpr::Named(name) => {
            named_to_typed(&name, info)
        }
        TypeExpr::Map { key, value } => {
            let key_type = type_expr_to_typed(*key, info);
            let value_type = type_expr_to_typed(*value, info);
            MapDescription { key_type, value_type, ..Default::default() }.into()
        }
        TypeExpr::Optional { element } => {
            let element_type = type_expr_to_typed(*element, info);
            OptionalDescription { element_type, ..Default::default() }.into()
        }
    }
}

/// 从原始类型转换为 XCellTyped
fn primitive_to_typed(p: ParserPrimitiveType, info: &TypeMetaInfo) -> XCellTyped {
    match p {
        ParserPrimitiveType::Bool => info.boolean.clone().into(),
        ParserPrimitiveType::I8 => IntegerDescription::range(i8::MIN, i8::MAX, IntegerKind::Integer8).into(),
        ParserPrimitiveType::I16 => IntegerDescription::range(i16::MIN, i16::MAX, IntegerKind::Integer16).into(),
        ParserPrimitiveType::I32 => IntegerDescription::range(i32::MIN, i32::MAX, IntegerKind::Integer32).into(),
        ParserPrimitiveType::I64 => IntegerDescription::range(i64::MIN, i64::MAX, IntegerKind::Integer64).into(),
        ParserPrimitiveType::U8 => IntegerDescription::range(u8::MIN, u8::MAX, IntegerKind::Unsigned8).into(),
        ParserPrimitiveType::U16 => IntegerDescription::range(u16::MIN, u16::MAX, IntegerKind::Unsigned16).into(),
        ParserPrimitiveType::U32 => IntegerDescription::range(u32::MIN, u32::MAX, IntegerKind::Unsigned32).into(),
        ParserPrimitiveType::U64 => IntegerDescription::range(u64::MIN, u64::MAX, IntegerKind::Unsigned64).into(),
        ParserPrimitiveType::F32 | ParserPrimitiveType::F64 => XCellTyped::Decimal(Default::default()),
        ParserPrimitiveType::String => info.string.clone().into(),
        ParserPrimitiveType::Utf8 | ParserPrimitiveType::Utf16 => info.string.clone().into(),
        ParserPrimitiveType::Color => XCellTyped::Color(Default::default()),
        ParserPrimitiveType::Time => XCellTyped::Time(Default::default()),
        ParserPrimitiveType::DateTime => XCellTyped::Time(Default::default()),
        ParserPrimitiveType::Date => XCellTyped::Time(Default::default()),
        ParserPrimitiveType::Vec2 => ArrayDescription::new(ArrayKind::Vector2).into(),
        ParserPrimitiveType::Vec3 => ArrayDescription::new(ArrayKind::Vector3).into(),
        ParserPrimitiveType::Vec4 => ArrayDescription::new(ArrayKind::Vector4).into(),
    }
}

/// 从命名类型转换为 XCellTyped
fn named_to_typed(name: &str, info: &TypeMetaInfo) -> XCellTyped {
    let normed = xcell_parser::norm_string(name);
    if info.string.matches_type(&normed) {
        return info.string.clone().into();
    }
    if let Some(s) = info.vector.matches_rest(name) {
        let typing = XCellTyped::parse(s, info);
        return info.vector.clone().with_type(typing).into();
    }
    EnumerateDescription::new(name).into()
}

/// 从泛型类型转换为 XCellTyped
fn generic_to_typed(name: &str, args: Vec<TypeExpr>, info: &TypeMetaInfo) -> XCellTyped {
    if let Some(s) = info.vector.matches_rest(name) {
        let typing = if args.is_empty() {
            XCellTyped::parse(s, info)
        } else {
            type_expr_to_typed(args.into_iter().next().unwrap(), info)
        };
        return info.vector.clone().with_type(typing).into();
    }
    EnumerateDescription::new(name).into()
}
