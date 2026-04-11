#![warn(missing_docs)]

#![doc = include_str!("../README.md")]

mod error;
mod ast;
mod parser;
mod utils;

pub use utils::norm_string;

pub use error::{ParseError, ParseErrorKind, ParseResult};
pub use ast::{
    FieldConstraint, FieldExpr, FieldMeta, MetaExpr, PrimitiveType, TableKind, TypeExpr, TypeMeta,
};

/// 解析类型表达式字符串
///
/// # 参数
/// * `input` - 类型表达式字符串
///
/// # 返回值
/// 成功时返回 `TypeExpr`，失败时返回 `ParseError`
pub fn parse_type(input: &str) -> ParseResult<TypeExpr> {
    parser::parse_type(input)
}

/// 解析字段表达式字符串
///
/// # 参数
/// * `input` - 字段表达式字符串，如 `field_name`, `@field_name`, `@@field_name`
///
/// # 返回值
/// 成功时返回 `FieldExpr`，失败时返回 `ParseError`
pub fn parse_field(input: &str) -> ParseResult<FieldExpr> {
    parser::parse_field(input)
}

/// 解析元数据表达式字符串
///
/// # 参数
/// * `input` - 元数据表达式字符串，如 `@dict`, `@class @unique(a, b)`
///
/// # 返回值
/// 成功时返回 `MetaExpr`，失败时返回 `ParseError`
pub fn parse_meta(input: &str) -> ParseResult<MetaExpr> {
    parser::parse_meta(input)
}

/// 解析字段元属性列表（Excel 注释中）
///
/// # 参数
/// * `input` - 元属性字符串，如 `@primary @default(0)`
///
/// # 返回值
/// 成功时返回 `Vec<FieldMeta>`，失败时返回 `ParseError`
pub fn parse_field_metas(input: &str) -> ParseResult<Vec<FieldMeta>> {
    parser::parse_field_metas(input)
}

/// 解析类型元属性列表（Excel 注释中）
///
/// # 参数
/// * `input` - 元属性字符串，如 `@min(0) @max(100)`
///
/// # 返回值
/// 成功时返回 `Vec<TypeMeta>`，失败时返回 `ParseError`
pub fn parse_type_metas(input: &str) -> ParseResult<Vec<TypeMeta>> {
    parser::parse_type_metas(input)
}

/// 解析数据表达式字符串
///
/// # 参数
/// * `input` - 数据表达式字符串
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 `ParseError`
pub fn parse_data(input: &str) -> ParseResult<String> {
    parser::parse_data(input)
}
