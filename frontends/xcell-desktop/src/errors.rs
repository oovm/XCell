//! XCell GUI 错误模块
//!
//! 定义应用错误类型和处理

use serde::Serialize;
use std::fmt;

/// XCell GUI 错误类型
#[derive(Debug, Serialize)]
pub enum XCellGuiError {
    /// 通用错误
    #[serde(rename = "general")]
    General(String),
    /// IO 错误
    #[serde(rename = "io")]
    Io(String),
    /// XCell 核心错误
    #[serde(rename = "xcell")]
    XCell(String),
}

impl fmt::Display for XCellGuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XCellGuiError::General(msg) => write!(f, "{}", msg),
            XCellGuiError::Io(msg) => write!(f, "IO error: {}", msg),
            XCellGuiError::XCell(msg) => write!(f, "XCell error: {}", msg),
        }
    }
}

impl std::error::Error for XCellGuiError {}

impl From<std::io::Error> for XCellGuiError {
    fn from(err: std::io::Error) -> Self {
        XCellGuiError::Io(err.to_string())
    }
}

impl From<xcell_analyzer::XError> for XCellGuiError {
    fn from(err: xcell_analyzer::XError) -> Self {
        XCellGuiError::XCell(err.to_string())
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, XCellGuiError>;
