use std::fmt;

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

/// 解析结果类型
pub type ParseResult<T> = Result<T, ParseError>;
