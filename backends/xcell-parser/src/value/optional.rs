//! 可选类型解析器
//!
//! 使用 nom 解析器组合子实现的可选类型解析器

use crate::error::ParseResult;

/// 解析可选值输入
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回 Option<String>，空值返回 None
pub fn parse_optional(input: &str) -> ParseResult<Option<String>> {
    if input.is_empty() {
        return Ok(None);
    }
    
    if is_empty_value(input) {
        return Ok(None);
    }
    
    Ok(Some(input.to_string()))
}

/// 解析可选值并应用解析函数
///
/// # 参数
/// * `input` - 输入字符串
/// * `parser` - 值解析函数
///
/// # 返回值
/// 成功时返回 Option<T>，空值返回 None
pub fn parse_optional_with<T, F>(input: &str, parser: F) -> ParseResult<Option<T>>
where
    F: Fn(&str) -> ParseResult<T>,
{
    if input.is_empty() {
        return Ok(None);
    }
    
    if is_empty_value(input) {
        return Ok(None);
    }
    
    match parser(input) {
        Ok(value) => Ok(Some(value)),
        Err(e) => Err(e),
    }
}

/// 检查是否为空值
fn is_empty_value(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower.is_empty() 
        || lower == "null" 
        || lower == "none" 
        || lower == "nil"
        || lower == "undefined"
        || lower == "n/a"
        || lower == "-"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_values() {
        assert_eq!(parse_optional("").unwrap(), None);
        assert_eq!(parse_optional("null").unwrap(), None);
        assert_eq!(parse_optional("NULL").unwrap(), None);
        assert_eq!(parse_optional("none").unwrap(), None);
        assert_eq!(parse_optional("None").unwrap(), None);
        assert_eq!(parse_optional("nil").unwrap(), None);
        assert_eq!(parse_optional("undefined").unwrap(), None);
        assert_eq!(parse_optional("n/a").unwrap(), None);
        assert_eq!(parse_optional("-").unwrap(), None);
    }

    #[test]
    fn test_non_empty_values() {
        assert_eq!(parse_optional("hello").unwrap(), Some("hello".to_string()));
        assert_eq!(parse_optional("123").unwrap(), Some("123".to_string()));
        assert_eq!(parse_optional("true").unwrap(), Some("true".to_string()));
    }

    #[test]
    fn test_with_parser() {
        let result = parse_optional_with("123", |s| Ok(s.parse::<i32>().unwrap())).unwrap();
        assert_eq!(result, Some(123));
        
        let result = parse_optional_with("", |s| Ok(s.parse::<i32>().unwrap())).unwrap();
        assert_eq!(result, None);
    }
}
