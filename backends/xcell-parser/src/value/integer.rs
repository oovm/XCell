//! 整数解析器
//!
//! 使用 nom 解析器组合子实现的整数解析器，支持：
//! - 十进制整数
//! - 十六进制整数（0x 前缀）
//! - 八进制整数（0o 前缀）
//! - 二进制整数（0b 前缀）
//! - 正负号

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1},
    combinator::{map, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

use crate::error::{IntegerParseErrorReason, ParseError, ParseErrorKind, ParseResult};

/// 解析整数输入并返回 i64
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回 (剩余输入, 解析结果)，失败时返回 ParseError
pub fn parse_integer(input: &str) -> ParseResult<i64> {
    match integer(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::InvalidSyntax(format!("整数后有意外的字符: {}", remaining)),
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::IntegerParseError {
                input: input.to_string(),
                reason: IntegerParseErrorReason::EmptyInput,
            },
            0,
        )),
    }
}

/// 解析整数输入并返回指定范围验证后的值
///
/// # 参数
/// * `input` - 输入字符串
/// * `min` - 最小值（可选）
/// * `max` - 最大值（可选）
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_integer_with_range(input: &str, min: Option<i64>, max: Option<i64>) -> ParseResult<i64> {
    let result = parse_integer(input)?;
    if let Some(mn) = min {
        if result < mn {
            return Err(ParseError::new(
                ParseErrorKind::RangeValidationError {
                    value: result.to_string(),
                    min: Some(mn.to_string()),
                    max: max.map(|m| m.to_string()),
                },
                0,
            ));
        }
    }
    if let Some(mx) = max {
        if result > mx {
            return Err(ParseError::new(
                ParseErrorKind::RangeValidationError {
                    value: result.to_string(),
                    min: min.map(|m| m.to_string()),
                    max: Some(mx.to_string()),
                },
                0,
            ));
        }
    }
    Ok(result)
}

/// 整数解析器
fn integer(input: &str) -> IResult<&str, i64, ParseError> {
    let (input, sign) = opt(alt((char('+'), char('-'))))(input)?;
    let sign_mult = match sign {
        Some('-') => -1i64,
        _ => 1i64,
    };
    
    let (input, num_str) = alt((
        |i| hexadecimal(i).map(|(r, s)| (r, s.to_string())),
        |i| octal(i).map(|(r, s)| (r, s.to_string())),
        |i| binary(i).map(|(r, s)| (r, s.to_string())),
        |i| decimal(i).map(|(r, s)| (r, s.to_string())),
    ))(input)?;
    
    let num = if num_str.starts_with("0x") || num_str.starts_with("0X") {
        i64::from_str_radix(&num_str[2..], 16)
            .map_err(|_| nom::Err::Error(ParseError::new(
                ParseErrorKind::IntegerParseError {
                    input: num_str.clone(),
                    reason: IntegerParseErrorReason::InvalidFormat,
                },
                0,
            )))?
    } else if num_str.starts_with("0o") || num_str.starts_with("0O") {
        i64::from_str_radix(&num_str[2..], 8)
            .map_err(|_| nom::Err::Error(ParseError::new(
                ParseErrorKind::IntegerParseError {
                    input: num_str.clone(),
                    reason: IntegerParseErrorReason::InvalidFormat,
                },
                0,
            )))?
    } else if num_str.starts_with("0b") || num_str.starts_with("0B") {
        i64::from_str_radix(&num_str[2..], 2)
            .map_err(|_| nom::Err::Error(ParseError::new(
                ParseErrorKind::IntegerParseError {
                    input: num_str.clone(),
                    reason: IntegerParseErrorReason::InvalidFormat,
                },
                0,
            )))?
    } else {
        num_str.parse::<i64>()
            .map_err(|_| nom::Err::Error(ParseError::new(
                ParseErrorKind::IntegerParseError {
                    input: num_str.clone(),
                    reason: IntegerParseErrorReason::InvalidFormat,
                },
                0,
            )))?
    };
    
    Ok((input, sign_mult * num))
}

/// 十进制整数解析器
fn decimal(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(digit1)(input)
}

/// 十六进制整数解析器（0x 前缀）
fn hexadecimal(input: &str) -> IResult<&str, &str, ParseError> {
    preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(take_while1(|c: char| c.is_ascii_hexdigit())),
    )(input)
}

/// 八进制整数解析器（0o 前缀）
fn octal(input: &str) -> IResult<&str, &str, ParseError> {
    preceded(
        alt((tag("0o"), tag("0O"))),
        recognize(take_while1(|c: char| c >= '0' && c <= '7')),
    )(input)
}

/// 二进制整数解析器（0b 前缀）
fn binary(input: &str) -> IResult<&str, &str, ParseError> {
    preceded(
        alt((tag("0b"), tag("0B"))),
        recognize(take_while1(|c: char| c == '0' || c == '1')),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(parse_integer("123").unwrap(), 123);
        assert_eq!(parse_integer("-456").unwrap(), -456);
        assert_eq!(parse_integer("+789").unwrap(), 789);
        assert_eq!(parse_integer("0").unwrap(), 0);
    }

    #[test]
    fn test_hexadecimal() {
        assert_eq!(parse_integer("0x1F").unwrap(), 31);
        assert_eq!(parse_integer("0xFF").unwrap(), 255);
        assert_eq!(parse_integer("0xABCD").unwrap(), 43981);
    }

    #[test]
    fn test_octal() {
        assert_eq!(parse_integer("0o77").unwrap(), 63);
        assert_eq!(parse_integer("0o123").unwrap(), 83);
    }

    #[test]
    fn test_binary() {
        assert_eq!(parse_integer("0b1010").unwrap(), 10);
        assert_eq!(parse_integer("0b11111111").unwrap(), 255);
    }

    #[test]
    fn test_range_validation() {
        assert!(parse_integer_with_range("5", Some(0), Some(10)).is_ok());
        assert!(parse_integer_with_range("-1", Some(0), Some(10)).is_err());
        assert!(parse_integer_with_range("11", Some(0), Some(10)).is_err());
    }
}
