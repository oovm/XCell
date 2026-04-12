//! 时间解析器
//!
//! 使用 nom 解析器组合子实现的时间解析器，支持：
//! - ISO 8601 日期时间格式
//! - 日期格式
//! - 时间格式

use nom::{
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::{char, digit1},
    combinator::{opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

use crate::error::{ParseError, ParseErrorKind, ParseResult};

/// 解析时间输入并返回 ISO 8601 格式字符串
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回解析后的时间字符串，失败时返回 ParseError
pub fn parse_time(input: &str) -> ParseResult<String> {
    if input.is_empty() {
        return Err(ParseError::new(
            ParseErrorKind::TimeParseError {
                input: input.to_string(),
                expected_format: Some("ISO 8601".to_string()),
            },
            0,
        ));
    }
    
    match time_value(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::TimeParseError {
                        input: input.to_string(),
                        expected_format: Some("ISO 8601".to_string()),
                    },
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::TimeParseError {
                input: input.to_string(),
                expected_format: Some("ISO 8601".to_string()),
            },
            0,
        )),
    }
}

/// 时间解析器
fn time_value(input: &str) -> IResult<&str, String, ParseError> {
    let (remaining, result) = recognize(alt((
        iso_datetime,
        iso_date,
        iso_time,
    )))(input)?;
    Ok((remaining, result.to_string()))
}

/// ISO 8601 日期时间格式 (YYYY-MM-DDTHH:MM:SS)
fn iso_datetime(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(tuple((
        date_part,
        alt((char('T'), char(' '))),
        time_part,
        opt(timezone_part),
    )))(input)
}

/// ISO 8601 日期格式 (YYYY-MM-DD)
fn iso_date(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(date_part)(input)
}

/// ISO 8601 时间格式 (HH:MM:SS)
fn iso_time(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(time_part)(input)
}

/// 日期部分
fn date_part(input: &str) -> IResult<&str, (&str, &str, &str), ParseError> {
    tuple((
        year,
        preceded(char('-'), month),
        preceded(char('-'), day),
    ))(input)
}

/// 时间部分
fn time_part(input: &str) -> IResult<&str, (&str, &str, &str), ParseError> {
    tuple((
        hour,
        preceded(char(':'), minute),
        preceded(char(':'), second),
    ))(input)
}

/// 时区部分
fn timezone_part(input: &str) -> IResult<&str, &str, ParseError> {
    alt((
        preceded(char('Z'), |i| Ok((i, "Z"))),
        preceded(
            alt((char('+'), char('-'))),
            recognize(tuple((hour, opt(preceded(char(':'), minute))))),
        ),
    ))(input)
}

/// 年份 (YYYY)
fn year(input: &str) -> IResult<&str, &str, ParseError> {
    take_while_m_n(4, 4, |c: char| c.is_ascii_digit())(input)
}

/// 月份 (MM)
fn month(input: &str) -> IResult<&str, &str, ParseError> {
    take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)
}

/// 日 (DD)
fn day(input: &str) -> IResult<&str, &str, ParseError> {
    take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)
}

/// 小时 (HH)
fn hour(input: &str) -> IResult<&str, &str, ParseError> {
    take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)
}

/// 分钟 (MM)
fn minute(input: &str) -> IResult<&str, &str, ParseError> {
    take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)
}

/// 秒 (SS)
fn second(input: &str) -> IResult<&str, &str, ParseError> {
    take_while_m_n(2, 2, |c: char| c.is_ascii_digit())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_date() {
        let result = parse_time("2024-01-15").unwrap();
        assert_eq!(result, "2024-01-15");
    }

    #[test]
    fn test_iso_time() {
        let result = parse_time("12:30:45").unwrap();
        assert_eq!(result, "12:30:45");
    }

    #[test]
    fn test_iso_datetime() {
        let result = parse_time("2024-01-15T12:30:45").unwrap();
        assert_eq!(result, "2024-01-15T12:30:45");
    }

    #[test]
    fn test_iso_datetime_with_space() {
        let result = parse_time("2024-01-15 12:30:45").unwrap();
        assert_eq!(result, "2024-01-15 12:30:45");
    }

    #[test]
    fn test_iso_datetime_with_timezone() {
        let result = parse_time("2024-01-15T12:30:45Z").unwrap();
        assert_eq!(result, "2024-01-15T12:30:45Z");
    }
}
