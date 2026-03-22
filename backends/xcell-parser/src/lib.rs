#![warn(missing_docs)]

//! XCell 类型表达式解析器
//!
//! 使用 parser combinator 实现的类型表达式解析器，支持复杂的嵌套类型语法。
//!
//! # 支持的类型语法
//!
//! - 原始类型: `i32`, `bool`, `string`, `f32`, etc.
//! - 引用类型: `&Item`, `ref<Monster>`
//! - 列表类型: `[i32]`, `[&Item]`
//! - 固定数组: `[i32; 5]`
//! - 泛型类型: `HashMap<string, i32>`
//! - 元组类型: `(i32, string)`
//! - 可选类型: `i32?`
//! - 独一类型: `@i32`, `@@i32` (主键)
//!
//! # 示例
//!
//! ```
//! use xcell_parser::parse_type;
//!
//! let ty = parse_type("i32").unwrap();
//! let ty = parse_type("[&Item]").unwrap();
//! let ty = parse_type("ref<Monster>").unwrap();
//! ```

mod error;
mod lexer;
mod ast;
mod parser;

pub use error::{ParseError, ParseErrorKind, ParseResult};
pub use lexer::{Lexer, Token, TokenKind};
pub use ast::{PrimitiveType, TypeExpr, TypeModifier};
pub use parser::TypeParser;

/// 解析类型表达式字符串
///
/// # 参数
/// * `input` - 类型表达式字符串
///
/// # 返回值
/// 成功时返回 `TypeExpr`，失败时返回 `ParseError`
///
/// # 示例
/// ```
/// use xcell_parser::parse_type;
///
/// let ty = parse_type("i32").unwrap();
/// let ty = parse_type("[&Item]").unwrap();
/// let ty = parse_type("ref<Monster>").unwrap();
/// ```
pub fn parse_type(input: &str) -> ParseResult<TypeExpr> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.collect::<Result<_, _>>()?;
    let mut parser = TypeParser::new(tokens);
    parser.parse_type_expr()
}

/// 解析类型表达式字符串，返回 AST 或错误消息
///
/// # 参数
/// * `input` - 类型表达式字符串
///
/// # 返回值
/// 成功时返回 `TypeExpr`，失败时返回错误消息字符串
pub fn parse_type_or_error(input: &str) -> Result<TypeExpr, String> {
    parse_type(input).map_err(|e| e.to_string())
}
