//! 颜色解析器
//!
//! 使用 nom 解析器组合子实现的颜色解析器，支持：
//! - 十六进制格式（#RGB, #RRGGBB, #RRGGBBAA）
//! - RGB 格式 rgb(r, g, b)
//! - RGBA 格式 rgba(r, g, b, a)
//! - 颜色名称

use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_while_m_n},
    character::complete::{char, digit1, space0},
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use crate::error::{ColorParseErrorReason, ParseError, ParseErrorKind, ParseResult};

/// RGBA 颜色值
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    /// 红色分量 (0-255)
    pub r: u8,
    /// 绿色分量 (0-255)
    pub g: u8,
    /// 蓝色分量 (0-255)
    pub b: u8,
    /// Alpha 分量 (0-255)
    pub a: u8,
}

impl Default for Rgba {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

/// 解析颜色输入
///
/// # 参数
/// * `input` - 输入字符串
///
/// # 返回值
/// 成功时返回 Rgba，失败时返回 ParseError
pub fn parse_color(input: &str) -> ParseResult<Rgba> {
    if input.is_empty() {
        return Err(ParseError::new(
            ParseErrorKind::ColorParseError {
                input: input.to_string(),
                reason: ColorParseErrorReason::EmptyInput,
            },
            0,
        ));
    }
    
    match color(input) {
        Ok((remaining, result)) => {
            if !remaining.is_empty() {
                Err(ParseError::new(
                    ParseErrorKind::ColorParseError {
                        input: input.to_string(),
                        reason: ColorParseErrorReason::InvalidHexFormat,
                    },
                    input.len() - remaining.len(),
                ))
            } else {
                Ok(result)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e),
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new(
            ParseErrorKind::ColorParseError {
                input: input.to_string(),
                reason: ColorParseErrorReason::InvalidHexFormat,
            },
            0,
        )),
    }
}

/// 颜色解析器
fn color(input: &str) -> IResult<&str, Rgba, ParseError> {
    alt((
        hex_color,
        rgb_color,
        rgba_color,
        named_color,
    ))(input)
}

/// 十六进制颜色解析器
fn hex_color(input: &str) -> IResult<&str, Rgba, ParseError> {
    preceded(
        char('#'),
        alt((
            hex8,
            hex6,
            hex4,
            hex3,
        )),
    )(input)
}

/// #RGB 格式
fn hex3(input: &str) -> IResult<&str, Rgba, ParseError> {
    let (remaining, s) = take_while_m_n(3, 3, |c: char| c.is_ascii_hexdigit())(input)?;
    let r = u8::from_str_radix(&s[0..1].repeat(2), 16).unwrap();
    let g = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap();
    let b = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap();
    Ok((remaining, Rgba { r, g, b, a: 255 }))
}

/// #RGBA 格式
fn hex4(input: &str) -> IResult<&str, Rgba, ParseError> {
    let (remaining, s) = take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit())(input)?;
    let r = u8::from_str_radix(&s[0..1].repeat(2), 16).unwrap();
    let g = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap();
    let b = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap();
    let a = u8::from_str_radix(&s[3..4].repeat(2), 16).unwrap();
    Ok((remaining, Rgba { r, g, b, a }))
}

/// #RRGGBB 格式
fn hex6(input: &str) -> IResult<&str, Rgba, ParseError> {
    let (remaining, s) = take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit())(input)?;
    let r = u8::from_str_radix(&s[0..2], 16).unwrap();
    let g = u8::from_str_radix(&s[2..4], 16).unwrap();
    let b = u8::from_str_radix(&s[4..6], 16).unwrap();
    Ok((remaining, Rgba { r, g, b, a: 255 }))
}

/// #RRGGBBAA 格式
fn hex8(input: &str) -> IResult<&str, Rgba, ParseError> {
    let (remaining, s) = take_while_m_n(8, 8, |c: char| c.is_ascii_hexdigit())(input)?;
    let r = u8::from_str_radix(&s[0..2], 16).unwrap();
    let g = u8::from_str_radix(&s[2..4], 16).unwrap();
    let b = u8::from_str_radix(&s[4..6], 16).unwrap();
    let a = u8::from_str_radix(&s[6..8], 16).unwrap();
    Ok((remaining, Rgba { r, g, b, a }))
}

/// rgb(r, g, b) 格式
fn rgb_color(input: &str) -> IResult<&str, Rgba, ParseError> {
    let (input, _) = preceded(tag_no_case("rgb"), char('('))(input)?;
    let (input, r) = color_component(input)?;
    let (input, _) = tuple((space0, char(','), space0))(input)?;
    let (input, g) = color_component(input)?;
    let (input, _) = tuple((space0, char(','), space0))(input)?;
    let (input, b) = color_component(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Rgba { r, g, b, a: 255 }))
}

/// rgba(r, g, b, a) 格式
fn rgba_color(input: &str) -> IResult<&str, Rgba, ParseError> {
    let (input, _) = preceded(tag_no_case("rgba"), char('('))(input)?;
    let (input, r) = color_component(input)?;
    let (input, _) = tuple((space0, char(','), space0))(input)?;
    let (input, g) = color_component(input)?;
    let (input, _) = tuple((space0, char(','), space0))(input)?;
    let (input, b) = color_component(input)?;
    let (input, _) = tuple((space0, char(','), space0))(input)?;
    let (input, a) = color_component(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Rgba { r, g, b, a }))
}

/// 颜色分量解析器 (0-255)
fn color_component(input: &str) -> IResult<&str, u8, ParseError> {
    let (remaining, s) = digit1(input)?;
    let val: u32 = s.parse().unwrap();
    if val > 255 {
        return Err(nom::Err::Error(ParseError::new(
            ParseErrorKind::ColorParseError {
                input: s.to_string(),
                reason: ColorParseErrorReason::InvalidRgbFormat,
            },
            0,
        )));
    }
    Ok((remaining, val as u8))
}

/// 命名颜色解析器
fn named_color(input: &str) -> IResult<&str, Rgba, ParseError> {
    let lower = input.to_lowercase();
    let color = match lower.as_str() {
        "red" => Rgba { r: 255, g: 0, b: 0, a: 255 },
        "green" => Rgba { r: 0, g: 128, b: 0, a: 255 },
        "blue" => Rgba { r: 0, g: 0, b: 255, a: 255 },
        "white" => Rgba { r: 255, g: 255, b: 255, a: 255 },
        "black" => Rgba { r: 0, g: 0, b: 0, a: 255 },
        "yellow" => Rgba { r: 255, g: 255, b: 0, a: 255 },
        "cyan" => Rgba { r: 0, g: 255, b: 255, a: 255 },
        "magenta" => Rgba { r: 255, g: 0, b: 255, a: 255 },
        "orange" => Rgba { r: 255, g: 165, b: 0, a: 255 },
        "purple" => Rgba { r: 128, g: 0, b: 128, a: 255 },
        "pink" => Rgba { r: 255, g: 192, b: 203, a: 255 },
        "gray" | "grey" => Rgba { r: 128, g: 128, b: 128, a: 255 },
        "transparent" => Rgba { r: 0, g: 0, b: 0, a: 0 },
        _ => return Err(nom::Err::Error(ParseError::new(
            ParseErrorKind::ColorParseError {
                input: input.to_string(),
                reason: ColorParseErrorReason::UnknownColorName,
            },
            0,
        ))),
    };
    Ok(("", color))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_colors() {
        let c = parse_color("#FF0000").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
        
        let c = parse_color("#F00").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_rgb_colors() {
        let c = parse_color("rgb(255, 0, 0)").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn test_rgba_colors() {
        let c = parse_color("rgba(255, 0, 0, 128)").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
        assert_eq!(c.a, 128);
    }

    #[test]
    fn test_named_colors() {
        let c = parse_color("red").unwrap();
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 0);
        assert_eq!(c.b, 0);
        
        let c = parse_color("transparent").unwrap();
        assert_eq!(c.a, 0);
    }
}
