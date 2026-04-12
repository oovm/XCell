//! 布尔解析器
//!
//! 使用 nom 解析器组合子实现的布尔解析器，支持：
//! - true/false（不区分大小写）
//! - 1/0
//! - yes/no（不区分大小写）

use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::char,
    combinator::{map, value},
    IResult,
};

use crate::error::{ParseError, ParseErrorKind, ParseResult};

/// 解析布尔输入并返回 bool
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_boolean(input: &str) -> ParseResult<bool> {
    match boolean(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::InvalidSyntax(format!("布尔值后有意外的字符: {}", remaining)),
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::BooleanParseError {
                input: input.to_string(),
            },
            0,
        )),
    }
}

/// 布尔解析器
fn boolean(input: &str) -> IResult<&str, bool, ParseError> {
    alt((
        true_value,
        false_value,
    ))(input)
}

/// 真值解析器
fn true_value(input: &str) -> IResult<&str, bool, ParseError> {
    alt((
        value(true, tag_no_case("true")),
        value(true, tag_no_case("yes")),
        value(true, char('1')),
    ))(input)
}

/// 假值解析器
fn false_value(input: &str) -> IResult<&str, bool, ParseError> {
    alt((
        value(false, tag_no_case("false")),
        value(false, tag_no_case("no")),
        value(false, char('0')),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_values() {
        assert_eq!(parse_boolean("true").unwrap(), true);
        assert_eq!(parse_boolean("TRUE").unwrap(), true);
        assert_eq!(parse_boolean("True").unwrap(), true);
        assert_eq!(parse_boolean("yes").unwrap(), true);
        assert_eq!(parse_boolean("YES").unwrap(), true);
        assert_eq!(parse_boolean("1").unwrap(), true);
    }

    #[test]
    fn test_false_values() {
        assert_eq!(parse_boolean("false").unwrap(), false);
        assert_eq!(parse_boolean("FALSE").unwrap(), false);
        assert_eq!(parse_boolean("False").unwrap(), false);
        assert_eq!(parse_boolean("no").unwrap(), false);
        assert_eq!(parse_boolean("NO").unwrap(), false);
        assert_eq!(parse_boolean("0").unwrap(), false);
    }

    #[test]
    fn test_invalid_values() {
        assert!(parse_boolean("invalid").is_err());
        assert!(parse_boolean("2").is_err());
    }
}
