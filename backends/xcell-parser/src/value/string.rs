//! 字符串解析器
//!
//! 使用 nom 解析器组合子实现的字符串解析器，支持：
//! - 引号包裹的字符串
//! - 转义字符
//! - 原始字符串

use nom::{
    branch::alt,
    bytes::complete::escaped,
    character::complete::{char, none_of, one_of},
    sequence::delimited,
    IResult,
};

use crate::error::{ParseError, ParseErrorKind, ParseResult};

/// 解析字符串输入
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_string(input: &str) -> ParseResult<String> {
    if input.is_empty() {
        return Ok(String::new());
    }
    
    match string_value(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::InvalidSyntax(format!("字符串后有意外的字符: {}", remaining)),
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::InvalidSyntax("不完整的字符串输入".to_string()),
            0,
        )),
    }
}

/// 字符串解析器
fn string_value(input: &str) -> IResult<&str, String, ParseError> {
    alt((
        quoted_string,
        raw_string,
    ))(input)
}

/// 引号包裹的字符串解析器
fn quoted_string(input: &str) -> IResult<&str, String, ParseError> {
    alt((
        double_quoted_string,
        single_quoted_string,
    ))(input)
}

/// 双引号字符串解析器
fn double_quoted_string(input: &str) -> IResult<&str, String, ParseError> {
    let (remaining, s) = delimited(
        char('"'),
        escaped(
            none_of("\\\""),
            '\\',
            one_of("\\\"nrt0"),
        ),
        char('"'),
    )(input)?;
    Ok((remaining, process_escape_sequences(s)))
}

/// 单引号字符串解析器
fn single_quoted_string(input: &str) -> IResult<&str, String, ParseError> {
    let (remaining, s) = delimited(
        char('\''),
        escaped(
            none_of("\\'"),
            '\\',
            one_of("\\'nrt0"),
        ),
        char('\''),
    )(input)?;
    Ok((remaining, process_escape_sequences(s)))
}

/// 原始字符串解析器（不处理转义）
fn raw_string(input: &str) -> IResult<&str, String, ParseError> {
    Ok(("", input.to_string()))
}

/// 处理转义序列
fn process_escape_sequences(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next) = chars.peek() {
                match next {
                    'n' => { result.push('\n'); chars.next(); }
                    'r' => { result.push('\r'); chars.next(); }
                    't' => { result.push('\t'); chars.next(); }
                    '0' => { result.push('\0'); chars.next(); }
                    '\\' => { result.push('\\'); chars.next(); }
                    '"' => { result.push('"'); chars.next(); }
                    '\'' => { result.push('\''); chars.next(); }
                    _ => { result.push(c); }
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quoted_string() {
        assert_eq!(parse_string("\"hello\"").unwrap(), "hello");
        assert_eq!(parse_string("'world'").unwrap(), "world");
    }

    #[test]
    fn test_escape_sequences() {
        assert_eq!(parse_string("\"hello\\nworld\"").unwrap(), "hello\nworld");
        assert_eq!(parse_string("\"tab\\there\"").unwrap(), "tab\there");
        assert_eq!(parse_string("\"quote\\\"here\"").unwrap(), "quote\"here");
    }

    #[test]
    fn test_raw_string() {
        assert_eq!(parse_string("hello world").unwrap(), "hello world");
        assert_eq!(parse_string("").unwrap(), "");
    }
}
