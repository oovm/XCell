use nom::{IResult, combinator::*, sequence::*, character::complete::*, multi::*, branch::alt, bytes::complete::{tag, tag_no_case, take_while, take_while1}};
use crate::error::{ParseError, ParseErrorKind, ParseResult};
use crate::ast::{FieldConstraint, FieldExpr, FieldMeta, MetaExpr, PrimitiveType, TableKind, TypeExpr, TypeMeta};

/// 解析类型表达式
pub fn parse_type(input: &str) -> ParseResult<TypeExpr> {
    match type_expr(input) {
        Ok((remaining, result)) => {
            if !remaining.trim().is_empty() {
                Err(ParseError::new(ParseErrorKind::InvalidSyntax("Unexpected input after type expression".to_string()), input.len() - remaining.len()))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(ParseErrorKind::InvalidSyntax("Incomplete input".to_string()), input.len())),
    }
}

/// 解析字段表达式
pub fn parse_field(input: &str) -> ParseResult<FieldExpr> {
    match field_expr(input) {
        Ok((remaining, result)) => {
            if !remaining.trim().is_empty() {
                Err(ParseError::new(ParseErrorKind::InvalidSyntax("Unexpected input after field expression".to_string()), input.len() - remaining.len()))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(ParseErrorKind::InvalidSyntax("Incomplete input".to_string()), input.len())),
    }
}

/// 解析元数据表达式
pub fn parse_meta(input: &str) -> ParseResult<MetaExpr> {
    match meta_expr(input) {
        Ok((remaining, result)) => {
            if !remaining.trim().is_empty() {
                Err(ParseError::new(ParseErrorKind::InvalidSyntax("Unexpected input after meta expression".to_string()), input.len() - remaining.len()))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(ParseErrorKind::InvalidSyntax("Incomplete input".to_string()), input.len())),
    }
}

/// 解析字段元属性列表
pub fn parse_field_metas(input: &str) -> ParseResult<Vec<FieldMeta>> {
    match many0(field_meta)(input) {
        Ok((remaining, result)) => {
            if !remaining.trim().is_empty() {
                Err(ParseError::new(ParseErrorKind::InvalidSyntax("Unexpected input after field metas".to_string()), input.len() - remaining.len()))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(ParseErrorKind::InvalidSyntax("Incomplete input".to_string()), input.len())),
    }
}

/// 解析类型元属性列表
pub fn parse_type_metas(input: &str) -> ParseResult<Vec<TypeMeta>> {
    match many0(type_meta)(input) {
        Ok((remaining, result)) => {
            if !remaining.trim().is_empty() {
                Err(ParseError::new(ParseErrorKind::InvalidSyntax("Unexpected input after type metas".to_string()), input.len() - remaining.len()))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(ParseErrorKind::InvalidSyntax("Incomplete input".to_string()), input.len())),
    }
}

/// 解析数据表达式
pub fn parse_data(input: &str) -> ParseResult<String> {
    match data_expr(input) {
        Ok((remaining, result)) => {
            if !remaining.trim().is_empty() {
                Err(ParseError::new(ParseErrorKind::InvalidSyntax("Unexpected input after data expression".to_string()), input.len() - remaining.len()))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(ParseErrorKind::InvalidSyntax("Incomplete input".to_string()), input.len())),
    }
}

/// 类型表达式解析器
fn type_expr(input: &str) -> IResult<&str, TypeExpr, ParseError> {
    let (input, ty) = primary_type(input)?;
    let (input, result) = many0(|i| {
        let (i, _) = char('?')(i)?;
        Ok((i, ()))
    })(input)?;
    
    let mut result_ty = ty;
    for _ in result {
        result_ty = TypeExpr::Optional { element: Box::new(result_ty) };
    }
    
    Ok((input, result_ty))
}

/// 基本类型解析器
fn primary_type(input: &str) -> IResult<&str, TypeExpr, ParseError> {
    alt((
        reference_type,
        list_or_array,
        tuple_type,
        named_or_generic,
    ))(input)
}

/// 引用类型解析器 `&TableName`
fn reference_type(input: &str) -> IResult<&str, TypeExpr, ParseError> {
    let (input, _) = char('&')(input)?;
    let (input, name) = identifier(input)?;
    Ok((input, TypeExpr::Reference { target: name.to_string() }))
}

/// 列表或固定数组解析器 `[T]` 或 `[T; N]`
fn list_or_array(input: &str) -> IResult<&str, TypeExpr, ParseError> {
    let (input, _) = char('[')(input)?;
    let (input, element) = type_expr(input)?;
    
    if let Ok((input, _)) = char::<&str, ParseError>(';')(input) {
        let (input, _) = space0(input)?;
        let (input, len_str) = digit1(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = char(']')(input)?;
        let length: usize = len_str.parse().unwrap();
        Ok((input, TypeExpr::FixedArray { 
            element: Box::new(element), 
            length 
        }))
    } else {
        let (input, _) = char(']')(input)?;
        Ok((input, TypeExpr::List { element: Box::new(element) }))
    }
}

/// 元组类型解析器 `(T1, T2, ...)`
fn tuple_type(input: &str) -> IResult<&str, TypeExpr, ParseError> {
    let (input, _) = char('(')(input)?;
    let (input, elements) = separated_list0(
        tuple((space0, char(','), space0)),
        type_expr
    )(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, TypeExpr::Tuple(elements)))
}

/// 命名类型或泛型类型解析器
fn named_or_generic(input: &str) -> IResult<&str, TypeExpr, ParseError> {
    let (input, name) = identifier(input)?;
    
    // 检查是否为原始类型
    if let Some(prim) = PrimitiveType::from_str(name) {
        return Ok((input, TypeExpr::Primitive(prim)));
    }
    
    // 检查是否为泛型 `Name<T1, T2, ...>`
    alt((
        // 泛型类型
        map(
            tuple((
                char('<'),
                space0,
                separated_list0(
                    tuple((space0, char(','), space0)),
                    type_expr
                ),
                space0,
                char('>')
            )),
            move |(_, _, args, _, _)| {
                // 特殊处理 Vec<T>
                if name.eq_ignore_ascii_case("Vec") && args.len() == 1 {
                    TypeExpr::Vec { element: Box::new(args[0].clone()) }
                } 
                // 特殊处理 Map<K, V>
                else if name.eq_ignore_ascii_case("Map") && args.len() == 2 {
                    TypeExpr::Map { 
                        key: Box::new(args[0].clone()), 
                        value: Box::new(args[1].clone()) 
                    }
                }
                // 普通泛型类型
                else {
                    TypeExpr::Generic { 
                        name: name.to_string(), 
                        args: args
                    }
                }
            }
        ),
        // 普通命名类型
        value(TypeExpr::Named(name.to_string()), eof)
    ))(input)
}

/// 字段表达式解析器
fn field_expr(input: &str) -> IResult<&str, FieldExpr, ParseError> {
    alt((
        // 主键约束 `@@field_name`
        map(
            tuple((char('@'), char('@'), identifier)),
            |(_, _, name)| FieldExpr {
                name: name.to_string(),
                constraint: Some(FieldConstraint::Primary)
            }
        ),
        // 唯一约束 `@field_name`
        map(
            tuple((char('@'), identifier)),
            |(_, name)| FieldExpr {
                name: name.to_string(),
                constraint: Some(FieldConstraint::Unique)
            }
        ),
        // 无约束 `field_name`
        map(
            identifier,
            |name| FieldExpr {
                name: name.to_string(),
                constraint: None
            }
        )
    ))(input)
}

/// 元数据表达式解析器
fn meta_expr(input: &str) -> IResult<&str, MetaExpr, ParseError> {
    let (input, _) = char('@')(input)?;
    let (input, kind_str) = identifier(input)?;
    let kind = match kind_str.to_lowercase().as_str() {
        "dict" => TableKind::Dict,
        "class" => TableKind::Class,
        "enum" => TableKind::Enum,
        "lang" => TableKind::Lang,
        "config" => TableKind::Config,
        _ => return Err(nom::Err::Error(ParseError::new(ParseErrorKind::InvalidTypeName(kind_str.to_string()), 0)))
    };
    
    let (input, unique_fields) = opt(
        tuple((
            space1,
            char('@'),
            tag_no_case("unique"),
            char('('),
            space0,
            separated_list0(
                tuple((space0, char(','), space0)),
                identifier
            ),
            space0,
            char(')')
        ))
    )(input)?;
    
    let unique_fields = unique_fields.map_or(Vec::new(), |(_, _, _, _, _, fields, _, _)| {
        fields.iter().map(|f| f.to_string()).collect()
    });
    
    Ok((input, MetaExpr { kind, unique_fields }))
}

/// 字段元属性解析器
fn field_meta(input: &str) -> IResult<&str, FieldMeta, ParseError> {
    let (input, _) = char('@')(input)?;
    let (input, name) = identifier(input)?;
    
    match name.to_lowercase().as_str() {
        "primary" => Ok((input, FieldMeta::Primary)),
        "virtual" => Ok((input, FieldMeta::Virtual)),
        "computed" => Ok((input, FieldMeta::Computed)),
        "default" => {
            let (input, _) = char('(')(input)?;
            let (input, value) = meta_value(input)?;
            let (input, _) = char(')')(input)?;
            Ok((input, FieldMeta::Default(value)))
        }
        _ => Err(nom::Err::Error(ParseError::new(ParseErrorKind::InvalidTypeName(name.to_string()), 0)))
    }
}

/// 类型元属性解析器
fn type_meta(input: &str) -> IResult<&str, TypeMeta, ParseError> {
    let (input, _) = char('@')(input)?;
    let (input, name) = identifier(input)?;
    
    match name.to_lowercase().as_str() {
        "min" => {
            let (input, _) = char('(')(input)?;
            let (input, value) = meta_number(input)?;
            let (input, _) = char(')')(input)?;
            Ok((input, TypeMeta::Min(value)))
        }
        "max" => {
            let (input, _) = char('(')(input)?;
            let (input, value) = meta_number(input)?;
            let (input, _) = char(')')(input)?;
            Ok((input, TypeMeta::Max(value)))
        }
        "range" => {
            let (input, _) = char('(')(input)?;
            let (input, min) = meta_number(input)?;
            let (input, _) = tuple((space0, char(','), space0))(input)?;
            let (input, max) = meta_number(input)?;
            let (input, _) = char(')')(input)?;
            Ok((input, TypeMeta::Range(min, max)))
        }
        "default" => {
            let (input, _) = char('(')(input)?;
            let (input, value) = meta_value(input)?;
            let (input, _) = char(')')(input)?;
            Ok((input, TypeMeta::Default(value)))
        }
        _ => Err(nom::Err::Error(ParseError::new(ParseErrorKind::InvalidTypeName(name.to_string()), 0)))
    }
}

/// 数据表达式解析器
fn data_expr(input: &str) -> IResult<&str, String, ParseError> {
    // 简单实现：返回原始字符串
    Ok(("", input.to_string()))
}

/// 标识符解析器
fn identifier(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(
        tuple((
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        ))
    )(input)
}

/// 元属性值解析器
fn meta_value(input: &str) -> IResult<&str, String, ParseError> {
    alt((
        // 字符串字面量
        map(
            delimited(char('"'), take_while(|c| c != '"'), char('"')),
            |s: &str| s.to_string()
        ),
        // 标识符或数字
        map(identifier, |s| s.to_string())
    ))(input)
}

/// 元属性数字解析器
fn meta_number(input: &str) -> IResult<&str, i64, ParseError> {
    map(digit1, |s: &str| s.parse().unwrap())(input)
}

/// 空格解析器（包括换行符）
fn space0(input: &str) -> IResult<&str, &str, ParseError> {
    take_while(|c: char| c.is_whitespace())(input)
}

/// 至少一个空格解析器
fn space1(input: &str) -> IResult<&str, &str, ParseError> {
    take_while1(|c: char| c.is_whitespace())(input)
}

/// 结束解析器
fn eof(input: &str) -> IResult<&str, &str, ParseError> {
    take_while(|c: char| c.is_whitespace())(input)
}
