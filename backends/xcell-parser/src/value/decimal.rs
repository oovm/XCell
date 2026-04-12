//! 小数解析器
//!
//! 使用 nom 解析器组合子实现的小数解析器，支持：
//! - 普通小数格式
//! - 科学计数法

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, opt, recognize},
    sequence::{preceded, tuple},
    IResult,
};

use crate::error::{DecimalParseErrorReason, ParseError, ParseErrorKind, ParseResult};

/// 解析小数输入并返回 f64
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_decimal(input: &str) -> ParseResult<f64> {
    match decimal(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::InvalidSyntax(format!("小数后有意外的字符: {}", remaining)),
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::DecimalParseError {
                input: input.to_string(),
                reason: DecimalParseErrorReason::EmptyInput,
            },
            0,
        )),
    }
}

/// 解析小数输入并返回指定范围验证后的值
///
/// # 参数
/// * `input` - 输入字符串
/// * `min` - 最小值（可选）
/// * `max` - 最大值（可选）
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_decimal_with_range(input: &str, min: Option<f64>, max: Option<f64>) -> ParseResult<f64> {
    let result = parse_decimal(input)?;
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

/// 小数解析器
fn decimal(input: &str) -> IResult<&str, f64, ParseError> {
    let (input, sign) = opt(alt((char('+'), char('-'))))(input)?;
    let sign_mult = match sign {
        Some('-') => -1.0f64,
        _ => 1.0f64,
    };
    
    let (input, num_str) = recognize(alt((
        scientific_notation,
        float_with_decimal,
        integer_float,
    )))(input)?;
    
    let num = num_str.parse::<f64>()
        .map_err(|_| nom::Err::Error(ParseError::new(
            ParseErrorKind::DecimalParseError {
                input: num_str.to_string(),
                reason: DecimalParseErrorReason::InvalidFormat,
            },
            0,
        )))?;
    
    Ok((input, sign_mult * num))
}

/// 科学计数法解析器
fn scientific_notation(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(tuple((
        alt((float_with_decimal, integer_float)),
        alt((char('e'), char('E'))),
        opt(alt((char('+'), char('-')))),
        digit1,
    )))(input)
}

/// 带小数点的浮点数解析器
fn float_with_decimal(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(tuple((
        digit1,
        char('.'),
        digit1,
    )))(input)
}

/// 整数形式的浮点数解析器
fn integer_float(input: &str) -> IResult<&str, &str, ParseError> {
    digit1(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(parse_decimal("3.14").unwrap(), 3.14);
        assert_eq!(parse_decimal("-0.5").unwrap(), -0.5);
        assert_eq!(parse_decimal("123.456").unwrap(), 123.456);
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(parse_decimal("1e10").unwrap(), 1e10);
        assert_eq!(parse_decimal("1.5e-3").unwrap(), 1.5e-3);
        assert_eq!(parse_decimal("2.5E+5").unwrap(), 2.5e5);
    }

    #[test]
    fn test_range_validation() {
        assert!(parse_decimal_with_range("5.5", Some(0.0), Some(10.0)).is_ok());
        assert!(parse_decimal_with_range("-1.0", Some(0.0), Some(10.0)).is_err());
        assert!(parse_decimal_with_range("11.0", Some(0.0), Some(10.0)).is_err());
    }
}
