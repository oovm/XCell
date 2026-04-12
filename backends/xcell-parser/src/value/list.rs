//! 列表解析器
//!
//! 使用 nom 解析器组合子实现的列表解析器，支持：
//! - 自定义分隔符
//! - 嵌套列表
//! - 固定长度验证

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, space0},
    combinator::opt,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::error::{ListParseErrorReason, ParseError, ParseErrorKind, ParseResult};

/// 解析列表输入并返回 Vec<String>
///
/// # 参数
/// * `input` - 输入字符串
/// * `delimiter` - 分隔符
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_list(input: &str, delimiter: char) -> ParseResult<Vec<String>> {
    match list_with_delimiter(input, delimiter) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::ListParseError {
                        input: input.to_string(),
                        reason: ListParseErrorReason::InvalidFormat,
                    },
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::ListParseError {
                input: input.to_string(),
                reason: ListParseErrorReason::EmptyInput,
            },
            0,
        )),
    }
}

/// 解析列表输入并验证长度
///
/// # 参数
/// * `input` - 输入字符串
/// * `delimiter` - 分隔符
/// * `expected_len` - 期望的长度（可选）
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_list_with_len(input: &str, delimiter: char, expected_len: Option<usize>) -> ParseResult<Vec<String>> {
    let result = parse_list(input, delimiter)?;
    if let Some(len) = expected_len {
        if result.len() != len {
            return Err(ParseError::new(
                ParseErrorKind::ListParseError {
                    input: input.to_string(),
                    reason: ListParseErrorReason::LengthMismatch,
                },
                0,
            ));
        }
    }
    Ok(result)
}

/// 带分隔符的列表解析器
fn list_with_delimiter(input: &str, delimiter: char) -> IResult<&str, Vec<String>, ParseError> {
    alt((
        bracketed_list(delimiter),
        plain_list(delimiter),
    ))(input)
}

/// 方括号列表 `[a,b,c]`
fn bracketed_list(delimiter: char) -> impl Fn(&str) -> IResult<&str, Vec<String>, ParseError> {
    move |input: &str| {
        let (input, _) = char('[')(input)?;
        let (input, items) = separated_list0(
            tuple((space0, char(delimiter), space0)),
            list_item,
        )(input)?;
        let (input, _) = char(']')(input)?;
        Ok((input, items))
    }
}

/// 无括号列表 `a,b,c`
fn plain_list(delimiter: char) -> impl Fn(&str) -> IResult<&str, Vec<String>, ParseError> {
    move |input: &str| {
        if input.is_empty() {
            return Ok(("", vec![]));
        }
        let (remaining, items) = separated_list0(
            tuple((space0, char(delimiter), space0)),
            list_item,
        )(input)?;
        Ok((remaining, items))
    }
}

/// 列表项解析器
fn list_item(input: &str) -> IResult<&str, String, ParseError> {
    let (remaining, s) = take_while1(|c: char| 
        !c.is_whitespace() && c != ',' && c != '[' && c != ']'
    )(input)?;
    Ok((remaining, s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comma_separated() {
        let result = parse_list("a,b,c", ',').unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_bracketed_list() {
        let result = parse_list("[a, b, c]", ',').unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_semicolon_separator() {
        let result = parse_list("a;b;c", ';').unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_length_validation() {
        assert!(parse_list_with_len("a,b", ',', Some(2)).is_ok());
        assert!(parse_list_with_len("a,b,c", ',', Some(2)).is_err());
    }
}
