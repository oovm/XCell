use std::fmt;
use nom::error::{ParseError as NomParseError, ContextError, ErrorKind};

/// 解析错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    /// 意外的 token
    UnexpectedToken {
        /// 期望的 token 类型
        expected: String,
        /// 实际的 token
        found: Option<String>,
    },
    /// 无效的类型名
    InvalidTypeName(String),
    /// 无效的数字字面量
    InvalidNumberLiteral(String),
    /// 空输入
    EmptyInput,
    /// 无效的语法
    InvalidSyntax(String),
    /// Nom 错误
    NomError(ErrorKind),
    /// 整数解析错误
    IntegerParseError {
        /// 输入值
        input: String,
        /// 错误原因
        reason: IntegerParseErrorReason,
    },
    /// 小数解析错误
    DecimalParseError {
        /// 输入值
        input: String,
        /// 错误原因
        reason: DecimalParseErrorReason,
    },
    /// 布尔解析错误
    BooleanParseError {
        /// 输入值
        input: String,
    },
    /// 向量解析错误
    VectorParseError {
        /// 输入值
        input: String,
        /// 期望长度
        expected_len: Option<usize>,
        /// 实际长度
        actual_len: Option<usize>,
    },
    /// 颜色解析错误
    ColorParseError {
        /// 输入值
        input: String,
        /// 错误原因
        reason: ColorParseErrorReason,
    },
    /// 时间解析错误
    TimeParseError {
        /// 输入值
        input: String,
        /// 期望格式
        expected_format: Option<String>,
    },
    /// 列表解析错误
    ListParseError {
        /// 输入值
        input: String,
        /// 错误原因
        reason: ListParseErrorReason,
    },
    /// 映射解析错误
    MapParseError {
        /// 输入值
        input: String,
        /// 错误原因
        reason: MapParseErrorReason,
    },
    /// 范围验证错误
    RangeValidationError {
        /// 值
        value: String,
        /// 最小值
        min: Option<String>,
        /// 最大值
        max: Option<String>,
    },
    /// 类型不匹配错误
    TypeMismatchError {
        /// 期望类型
        expected: String,
        /// 实际值
        actual: String,
    },
}

/// 整数解析错误原因
#[derive(Debug, Clone, PartialEq)]
pub enum IntegerParseErrorReason {
    /// 无效的数字格式
    InvalidFormat,
    /// 超出范围
    OutOfRange,
    /// 无效的进制前缀
    InvalidRadixPrefix,
    /// 空输入
    EmptyInput,
}

/// 小数解析错误原因
#[derive(Debug, Clone, PartialEq)]
pub enum DecimalParseErrorReason {
    /// 无效的数字格式
    InvalidFormat,
    /// 超出范围
    OutOfRange,
    /// 无效的科学计数法
    InvalidScientificNotation,
    /// 空输入
    EmptyInput,
}

/// 颜色解析错误原因
#[derive(Debug, Clone, PartialEq)]
pub enum ColorParseErrorReason {
    /// 无效的十六进制格式
    InvalidHexFormat,
    /// 无效的 RGB 格式
    InvalidRgbFormat,
    /// 无效的 RGBA 格式
    InvalidRgbaFormat,
    /// 未知的颜色名称
    UnknownColorName,
    /// 空输入
    EmptyInput,
}

/// 列表解析错误原因
#[derive(Debug, Clone, PartialEq)]
pub enum ListParseErrorReason {
    /// 无效的列表格式
    InvalidFormat,
    /// 长度不匹配
    LengthMismatch,
    /// 嵌套解析错误
    NestedError,
    /// 空输入
    EmptyInput,
}

/// 映射解析错误原因
#[derive(Debug, Clone, PartialEq)]
pub enum MapParseErrorReason {
    /// 无效的映射格式
    InvalidFormat,
    /// 缺少键
    MissingKey,
    /// 缺少值
    MissingValue,
    /// 嵌套解析错误
    NestedError,
    /// 空输入
    EmptyInput,
}

/// 解析错误
#[derive(Debug, Clone)]
pub struct ParseError {
    /// 错误类型
    pub kind: ParseErrorKind,
    /// 错误位置（字节偏移）
    pub position: usize,
    /// 错误消息
    pub message: String,
    /// 错误行号（从 1 开始，0 表示未计算）
    pub line: usize,
    /// 错误列号（从 1 开始，0 表示未计算）
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line > 0 && self.column > 0 {
            write!(
                f,
                "解析错误 ({}:{} 位置 {}): {}",
                self.line, self.column, self.position, self.message
            )
        } else {
            write!(f, "解析错误 (位置 {}): {}", self.position, self.message)
        }
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    /// 创建新的解析错误
    pub fn new(kind: ParseErrorKind, position: usize) -> Self {
        let message = match &kind {
            ParseErrorKind::UnexpectedToken { expected, found } => {
                match found {
                    Some(f) => format!("期望 {}，但找到 {}", expected, f),
                    None => format!("期望 {}，但到达输入末尾", expected),
                }
            }
            ParseErrorKind::InvalidTypeName(name) => format!("无效的类型名: {}", name),
            ParseErrorKind::InvalidNumberLiteral(s) => format!("无效的数字字面量: {}", s),
            ParseErrorKind::EmptyInput => "输入为空".to_string(),
            ParseErrorKind::InvalidSyntax(msg) => msg.clone(),
            ParseErrorKind::NomError(kind) => format!("解析错误: {:?}", kind),
            ParseErrorKind::IntegerParseError { input, reason } => {
                match reason {
                    IntegerParseErrorReason::InvalidFormat => format!("无效的整数格式: {}", input),
                    IntegerParseErrorReason::OutOfRange => format!("整数超出范围: {}", input),
                    IntegerParseErrorReason::InvalidRadixPrefix => format!("无效的进制前缀: {}", input),
                    IntegerParseErrorReason::EmptyInput => "整数输入为空".to_string(),
                }
            }
            ParseErrorKind::DecimalParseError { input, reason } => {
                match reason {
                    DecimalParseErrorReason::InvalidFormat => format!("无效的小数格式: {}", input),
                    DecimalParseErrorReason::OutOfRange => format!("小数超出范围: {}", input),
                    DecimalParseErrorReason::InvalidScientificNotation => format!("无效的科学计数法: {}", input),
                    DecimalParseErrorReason::EmptyInput => "小数输入为空".to_string(),
                }
            }
            ParseErrorKind::BooleanParseError { input } => format!("无效的布尔值: {}", input),
            ParseErrorKind::VectorParseError { input, expected_len, actual_len } => {
                match (expected_len, actual_len) {
                    (Some(exp), Some(act)) => format!("向量长度不匹配: 期望 {} 个元素，实际 {} 个元素 (输入: {})", exp, act, input),
                    (Some(exp), None) => format!("向量解析失败: 期望 {} 个元素 (输入: {})", exp, input),
                    (None, Some(act)) => format!("向量解析失败: 实际 {} 个元素 (输入: {})", act, input),
                    (None, None) => format!("无效的向量格式: {}", input),
                }
            }
            ParseErrorKind::ColorParseError { input, reason } => {
                match reason {
                    ColorParseErrorReason::InvalidHexFormat => format!("无效的十六进制颜色格式: {}", input),
                    ColorParseErrorReason::InvalidRgbFormat => format!("无效的 RGB 颜色格式: {}", input),
                    ColorParseErrorReason::InvalidRgbaFormat => format!("无效的 RGBA 颜色格式: {}", input),
                    ColorParseErrorReason::UnknownColorName => format!("未知的颜色名称: {}", input),
                    ColorParseErrorReason::EmptyInput => "颜色输入为空".to_string(),
                }
            }
            ParseErrorKind::TimeParseError { input, expected_format } => {
                match expected_format {
                    Some(fmt) => format!("时间解析失败: 期望格式 {} (输入: {})", fmt, input),
                    None => format!("无效的时间格式: {}", input),
                }
            }
            ParseErrorKind::ListParseError { input, reason } => {
                match reason {
                    ListParseErrorReason::InvalidFormat => format!("无效的列表格式: {}", input),
                    ListParseErrorReason::LengthMismatch => format!("列表长度不匹配: {}", input),
                    ListParseErrorReason::NestedError => format!("列表嵌套解析错误: {}", input),
                    ListParseErrorReason::EmptyInput => "列表输入为空".to_string(),
                }
            }
            ParseErrorKind::MapParseError { input, reason } => {
                match reason {
                    MapParseErrorReason::InvalidFormat => format!("无效的映射格式: {}", input),
                    MapParseErrorReason::MissingKey => format!("映射缺少键: {}", input),
                    MapParseErrorReason::MissingValue => format!("映射缺少值: {}", input),
                    MapParseErrorReason::NestedError => format!("映射嵌套解析错误: {}", input),
                    MapParseErrorReason::EmptyInput => "映射输入为空".to_string(),
                }
            }
            ParseErrorKind::RangeValidationError { value, min, max } => {
                match (min, max) {
                    (Some(mn), Some(mx)) => format!("值 {} 超出范围 [{}, {}]", value, mn, mx),
                    (Some(mn), None) => format!("值 {} 小于最小值 {}", value, mn),
                    (None, Some(mx)) => format!("值 {} 大于最大值 {}", value, mx),
                    (None, None) => format!("值 {} 超出范围", value),
                }
            }
            ParseErrorKind::TypeMismatchError { expected, actual } => {
                format!("类型不匹配: 期望 {}, 实际 {}", expected, actual)
            }
        };
        Self {
            kind,
            position,
            message,
            line: 0,
            column: 0,
        }
    }

    /// 根据位置和源输入计算行号和列号
    pub fn from_position(kind: ParseErrorKind, position: usize, input: &str) -> Self {
        let mut error = Self::new(kind, position);
        let mut line = 1;
        let mut col = 1;
        for (i, ch) in input.char_indices() {
            if i >= position {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        error.line = line;
        error.column = col;
        error
    }

    /// 创建意外的 token 错误
    pub fn unexpected_token(expected: &str, found: Option<&str>, position: usize) -> Self {
        Self::new(
            ParseErrorKind::UnexpectedToken {
                expected: expected.to_string(),
                found: found.map(|s| s.to_string()),
            },
            position,
        )
    }

    /// 创建无效类型名错误
    pub fn invalid_type_name(name: &str, position: usize) -> Self {
        Self::new(ParseErrorKind::InvalidTypeName(name.to_string()), position)
    }
}

/// 实现 nom 的 ParseError trait
impl<'a> NomParseError<&'a str> for ParseError {
    fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
        Self::new(ParseErrorKind::NomError(kind), input.len())
    }

    fn append(_input: &'a str, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

/// 实现 nom 的 ContextError trait
impl<'a> ContextError<&'a str> for ParseError {
    fn add_context(_input: &'a str, ctx: &'static str, other: Self) -> Self {
        let new_kind = ParseErrorKind::InvalidSyntax(format!("{}: {}", ctx, other.message));
        Self::new(new_kind, other.position)
    }
}

/// 实现 nom 的 FromExternalError trait for Self
impl<'a> nom::error::FromExternalError<&'a str, Self> for ParseError {
    fn from_external_error(_input: &'a str, _kind: ErrorKind, e: Self) -> Self {
        e
    }
}

/// 解析结果类型
pub type ParseResult<T> = Result<T, ParseError>;
