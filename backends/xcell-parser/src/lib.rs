#![warn(missing_docs)]

#![doc = include_str!("../README.md")]

mod error;
mod lexer;
mod ast;
mod parser;

pub use error::{ParseError, ParseErrorKind, ParseResult};
pub use lexer::{Lexer, Token, TokenKind};
pub use ast::{
    FieldConstraint, FieldExpr, FieldMeta, MetaExpr, PrimitiveType, TableKind, TypeExpr, TypeMeta,
};
pub use parser::TypeParser;

/// 解析类型表达式字符串
///
/// # 参数
/// * `input` - 类型表达式字符串
///
/// # 返回值
/// 成功时返回 `TypeExpr`，失败时返回 `ParseError`
pub fn parse_type(input: &str) -> ParseResult<TypeExpr> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_type_expr()
}

/// 解析字段表达式字符串
///
/// # 参数
/// * `input` - 字段表达式字符串，如 `field_name`, `@field_name`, `@@field_name`
///
/// # 返回值
/// 成功时返回 `FieldExpr`，失败时返回 `ParseError`
pub fn parse_field(input: &str) -> ParseResult<FieldExpr> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_field_expr()
}

/// 解析元数据表达式字符串
///
/// # 参数
/// * `input` - 元数据表达式字符串，如 `@dict`, `@class @unique(a, b)`
///
/// # 返回值
/// 成功时返回 `MetaExpr`，失败时返回 `ParseError`
pub fn parse_meta(input: &str) -> ParseResult<MetaExpr> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_meta_expr()
}

/// 解析字段元属性列表（Excel 注释中）
///
/// # 参数
/// * `input` - 元属性字符串，如 `@primary @default(0)`
///
/// # 返回值
/// 成功时返回 `Vec<FieldMeta>`，失败时返回 `ParseError`
pub fn parse_field_metas(input: &str) -> ParseResult<Vec<FieldMeta>> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_field_metas()
}

/// 解析类型元属性列表（Excel 注释中）
///
/// # 参数
/// * `input` - 元属性字符串，如 `@min(0) @max(100)`
///
/// # 返回值
/// 成功时返回 `Vec<TypeMeta>`，失败时返回 `ParseError`
pub fn parse_type_metas(input: &str) -> ParseResult<Vec<TypeMeta>> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_type_metas()
}
