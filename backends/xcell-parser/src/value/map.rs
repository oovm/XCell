//! 映射解析器
//!
//! 使用 nom 解析器组合子实现的映射解析器，支持：
//! - `{key: value}` 格式
//! - 自定义键值分隔符

use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, space0},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::error::{MapParseErrorReason, ParseError, ParseErrorKind, ParseResult};

/// 键值对
pub type KeyValuePair = (String, String);

/// 解析映射输入并返回 Vec<KeyValuePair>
///
/// # 参数
/// * `input` - 输入字符串
/// * `key_value_sep` - 键值分隔符
/// * `entry_sep` - 条目分隔符
///
/// # 返回值
/// 成功时返回解析结果，失败时返回 ParseError
pub fn parse_map(input: &str, key_value_sep: char, entry_sep: char) -> ParseResult<Vec<KeyValuePair>> {
    match map_value(input, key_value_sep, entry_sep) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::MapParseError {
                        input: input.to_string(),
                        reason: MapParseErrorReason::InvalidFormat,
                    },
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::MapParseError {
                input: input.to_string(),
                reason: MapParseErrorReason::EmptyInput,
            },
            0,
        )),
    }
}

/// 映射解析器
fn map_value(input: &str, key_value_sep: char, entry_sep: char) -> IResult<&str, Vec<KeyValuePair>, ParseError> {
    alt((
        braced_map(key_value_sep, entry_sep),
        plain_map(key_value_sep, entry_sep),
    ))(input)
}

/// 花括号映射 `{key: value, ...}`
fn braced_map(key_value_sep: char, entry_sep: char) -> impl Fn(&str) -> IResult<&str, Vec<KeyValuePair>, ParseError> {
    move |input: &str| {
        let (input, _) = char('{')(input)?;
        let (input, items) = separated_list0(
            tuple((space0, char(entry_sep), space0)),
            key_value_pair(key_value_sep),
        )(input)?;
        let (input, _) = char('}')(input)?;
        Ok((input, items))
    }
}

/// 无括号映射 `key: value, ...`
fn plain_map(key_value_sep: char, entry_sep: char) -> impl Fn(&str) -> IResult<&str, Vec<KeyValuePair>, ParseError> {
    move |input: &str| {
        if input.is_empty() {
            return Ok(("", vec![]));
        }
        separated_list0(
            tuple((space0, char(entry_sep), space0)),
            key_value_pair(key_value_sep),
        )(input)
    }
}

/// 键值对解析器
fn key_value_pair(sep: char) -> impl Fn(&str) -> IResult<&str, KeyValuePair, ParseError> {
    move |input: &str| {
        let (input, key) = take_while1(|c: char| !c.is_whitespace() && c != sep && c != ',' && c != '{' && c != '}')(input)?;
        let (input, _) = tuple((space0, char(sep), space0))(input)?;
        let (input, value) = take_while1(|c: char| !c.is_whitespace() && c != ',' && c != '{' && c != '}')(input)?;
        Ok((input, (key.to_string(), value.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braced_map() {
        let result = parse_map("{a:1, b:2}", ':', ',').unwrap();
        assert_eq!(result, vec![("a".to_string(), "1".to_string()), ("b".to_string(), "2".to_string())]);
    }

    #[test]
    fn test_plain_map() {
        let result = parse_map("a:1, b:2", ':', ',').unwrap();
        assert_eq!(result, vec![("a".to_string(), "1".to_string()), ("b".to_string(), "2".to_string())]);
    }

    #[test]
    fn test_custom_separators() {
        let result = parse_map("a=1; b=2", '=', ';').unwrap();
        assert_eq!(result, vec![("a".to_string(), "1".to_string()), ("b".to_string(), "2".to_string())]);
    }
}
