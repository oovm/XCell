#![warn(missing_docs)]

//! XCell 类型表达式解析器
//!
//! 使用 parser combinator 实现的类型表达式解析器，支持复杂的嵌套类型语法。
//!
//! # 解析功能
//!
//! - `parse_type` - 解析类型表达式（如 `i32`, `&Item`, `[string]`）
//! - `parse_field` - 解析字段名（可能包含 `@` 或 `@@` 约束前缀）
//! - `parse_meta` - 解析元数据（第一行第一个单元格，如 `@dict`, `@class`）
//! - `parse_field_metas` - 解析字段元属性（Excel 注释中）
//! - `parse_type_metas` - 解析类型元属性（Excel 注释中）
//!
//! # 支持的类型语法
//!
//! - 原始类型: `i32`, `bool`, `string`, `f32`, etc.
//! - 引用类型: `&TableName`
//! - 列表类型: `[T]`, `[&Item]`
//! - 固定数组: `[T; N]`
//! - 向量类型: `Vec<T>`
//! - 泛型类型: `HashMap<string, i32>`
//!
//! # 示例
//!
//! ```
//! use xcell_parser::{parse_type, parse_field, parse_meta};
//!
//! // 解析类型
//! let ty = parse_type("i32").unwrap();
//! let ty = parse_type("[&Item]").unwrap();
//!
//! // 解析字段（带约束）
//! let field = parse_field("@email").unwrap();  // 唯一约束
//! let field = parse_field("@@id").unwrap();    // 主键约束
//!
//! // 解析元数据
//! let meta = parse_meta("@dict").unwrap();
//! let meta = parse_meta("@class @unique(name, level)").unwrap();
//! ```

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
