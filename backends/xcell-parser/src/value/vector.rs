//! 向量解析器
//!
//! 使用 nom 解析器组合子实现的向量解析器，支持：
//! - `[a,b,c]` 格式
//! - `(a,b,c)` 格式
//! - `a,b,c` 格式（无括号）

use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::{opt, recognize},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::error::{ParseError, ParseErrorKind, ParseResult};

/// 解析向量输入并返回 Vec<f64>
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_vector(input: &str) -> ParseResult<Vec<f64>> {
    match vector(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::VectorParseError {
                        input: input.to_string(),
                        expected_len: None,
                        actual_len: None,
                    },
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::VectorParseError {
                input: input.to_string(),
                expected_len: None,
                actual_len: None,
            },
            0,
        )),
    }
}

/// 解析向量输入并验证长度
///
/// # 参数
/// * `input` - 输入字符串
/// * `expected_len` - 期望的长度
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_vector_with_len(input: &str, expected_len: usize) -> ParseResult<Vec<f64>> {
    let result = parse_vector(input)?;
    if result.len() != expected_len {
        return Err(ParseError::new(
            ParseErrorKind::VectorParseError {
                input: input.to_string(),
                expected_len: Some(expected_len),
                actual_len: Some(result.len()),
            },
            0,
        ));
    }
    Ok(result)
}

/// 向量解析器
fn vector(input: &str) -> IResult<&str, Vec<f64>, ParseError> {
    alt((
        bracketed_vector,
        parenthesized_vector,
        plain_vector,
    ))(input)
}

/// 方括号向量 `[a,b,c]`
fn bracketed_vector(input: &str) -> IResult<&str, Vec<f64>, ParseError> {
    let (input, _) = char('[')(input)?;
    let (input, items) = separated_list0(
        tuple((space0, char(','), space0)),
        number_str,
    )(input)?;
    let (input, _) = char(']')(input)?;
    
    let result: Vec<f64> = items.into_iter().filter_map(|s| s.parse().ok()).collect();
    Ok((input, result))
}

/// 圆括号向量 `(a,b,c)`
fn parenthesized_vector(input: &str) -> IResult<&str, Vec<f64>, ParseError> {
    let (input, _) = char('(')(input)?;
    let (input, items) = separated_list0(
        tuple((space0, char(','), space0)),
        number_str,
    )(input)?;
    let (input, _) = char(')')(input)?;
    
    let result: Vec<f64> = items.into_iter().filter_map(|s| s.parse().ok()).collect();
    Ok((input, result))
}

/// 无括号向量 `a,b,c`
fn plain_vector(input: &str) -> IResult<&str, Vec<f64>, ParseError> {
    if input.is_empty() {
        return Ok(("", vec![]));
    }
    
    let (remaining, items) = separated_list0(
        tuple((space0, char(','), space0)),
        number_str,
    )(input)?;
    
    let result: Vec<f64> = items.into_iter().filter_map(|s| s.parse().ok()).collect();
    Ok((remaining, result))
}

/// 数字字符串解析器
fn number_str(input: &str) -> IResult<&str, &str, ParseError> {
    recognize(tuple((
        opt(char('-')),
        digit1,
        opt(tuple((char('.'), digit1))),
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracketed_vector() {
        let result = parse_vector("[1,2,3]").unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_parenthesized_vector() {
        let result = parse_vector("(1.5, 2.5, 3.5)").unwrap();
        assert_eq!(result, vec![1.5, 2.5, 3.5]);
    }

    #[test]
    fn test_plain_vector() {
        let result = parse_vector("1,2,3").unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_negative_values() {
        let result = parse_vector("[-10,10]").unwrap();
        assert_eq!(result, vec![-10.0, 10.0]);
    }

    #[test]
    fn test_length_validation() {
        assert!(parse_vector_with_len("[1,2]", 2).is_ok());
        assert!(parse_vector_with_len("[1,2,3]", 2).is_err());
    }
}
