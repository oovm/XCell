use super::*;
use xcell_parser::{TypeExpr, PrimitiveType as ParserPrimitiveType};

impl XCellTyped {
    /// 使用 xcell-parser 解析类型表达式
    pub fn parse(input: &str, info: &TypeMetaInfo) -> Self {
        match xcell_parser::parse_type(input) {
            Ok(expr) => Self::from_type_expr(expr, info),
            Err(_) => {
                // 回退到旧的解析逻辑，处理一些特殊情况
                Self::parse_fallback(input, info)
            }
        }
    }

    /// 从 TypeExpr 转换为 XCellTyped
    fn from_type_expr(expr: TypeExpr, info: &TypeMetaInfo) -> Self {
        match expr {
            TypeExpr::Primitive(p) => Self::from_primitive(p, info),
            TypeExpr::Reference { target } => ReferenceDescription::new(&target).into(),
            TypeExpr::List { element } => {
                let element_type = Self::from_type_expr(*element, info);
                ListDescription { element_type, ..Default::default() }.into()
            }
            TypeExpr::FixedArray { element, length } => {
                let element_type = Self::from_type_expr(*element, info);
                ListDescription {
                    element_type,
                    fixed_length: Some(length),
                    ..Default::default()
                }.into()
            }
            TypeExpr::Vec { element } => {
                let element_type = Self::from_type_expr(*element, info);
                info.vector.clone().with_type(element_type).into()
            }
            TypeExpr::Generic { name, args } => {
                Self::from_generic(&name, args, info)
            }
            TypeExpr::Tuple(elements) => {
                // 元组类型暂不直接支持
                if elements.is_empty() {
                    info.string.clone().into()
                } else {
                    Self::from_type_expr(elements.into_iter().next().unwrap(), info)
                }
            }
            TypeExpr::Named(name) => {
                Self::from_named(&name, info)
            }
        }
    }

    /// 从原始类型转换
    fn from_primitive(p: ParserPrimitiveType, info: &TypeMetaInfo) -> Self {
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
            ParserPrimitiveType::F32 | ParserPrimitiveType::F64 => Self::Decimal(Default::default()),
            ParserPrimitiveType::String => info.string.clone().into(),
            ParserPrimitiveType::Utf8 | ParserPrimitiveType::Utf16 => info.string.clone().into(),
            ParserPrimitiveType::Color => Self::Color(Default::default()),
            ParserPrimitiveType::Time => Self::Time(Default::default()),
            ParserPrimitiveType::DateTime => Self::Time(Default::default()),
            ParserPrimitiveType::Date => Self::Time(Default::default()),
            ParserPrimitiveType::Vec2 => ArrayDescription::new(ArrayKind::Vector2).into(),
            ParserPrimitiveType::Vec3 => ArrayDescription::new(ArrayKind::Vector3).into(),
            ParserPrimitiveType::Vec4 => ArrayDescription::new(ArrayKind::Vector4).into(),
        }
    }

    /// 从命名类型转换
    fn from_named(name: &str, info: &TypeMetaInfo) -> Self {
        let normed = name.to_lowercase();
        if info.matches_string(&normed) {
            return info.string.clone().into();
        }
        if let Some(s) = info.matches_vector(name) {
            let typing = XCellTyped::parse(s, info);
            return info.vector.clone().with_type(typing).into();
        }
        EnumerateDescription::new(name).into()
    }

    /// 从泛型类型转换
    fn from_generic(name: &str, args: Vec<TypeExpr>, info: &TypeMetaInfo) -> Self {
        // 处理 vector 类型
        if let Some(s) = info.matches_vector(name) {
            let typing = if args.is_empty() {
                XCellTyped::parse(s, info)
            } else {
                Self::from_type_expr(args.into_iter().next().unwrap(), info)
            };
            return info.vector.clone().with_type(typing).into();
        }
        // 其他泛型类型暂不支持，返回命名类型
        EnumerateDescription::new(name).into()
    }

    /// 回退解析逻辑
    fn parse_fallback(input: &str, info: &TypeMetaInfo) -> Self {
        let trimmed = input.trim();
        let normed = Self::norm_typing(trimmed);
        
        if info.matches_string(&normed) {
            return info.string.clone().into();
        }
        if let Some(s) = info.matches_vector(trimmed) {
            let typing = XCellTyped::parse(s, info);
            return info.vector.clone().with_type(typing).into();
        }
        EnumerateDescription::new(trimmed).into()
    }

    fn norm_typing(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for c in input.chars() {
            if c.is_ascii_uppercase() {
                out.push(c.to_ascii_lowercase())
            }
            else if c.is_ascii_whitespace() {
            }
            else {
                out.push(c)
            }
        }
        out
    }
}

impl TypeMetaInfo {
    fn matches_string(&self, s: &str) -> bool {
        self.string.matches_type(s)
    }
    fn matches_vector<'i>(&self, raw: &'i str) -> Option<&'i str> {
        return self.vector.matches_rest(raw);
    }
}
