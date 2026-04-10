//! XCell GUI 错误模块
//!
//! 定义应用错误类型和处理

/// XCell GUI 错误类型
#[derive(Debug, thiserror::Error)]
pub enum XCellGuiError {
    /// 通用错误
    #[error("{0}")]
    General(String),
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// XCell 核心错误
    #[error("XCell error: {0}")]
    XCell(String),
}

impl From<xcell_analyzer::XError> for XCellGuiError {
    fn from(err: xcell_analyzer::XError) -> Self {
        XCellGuiError::XCell(err.to_string())
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, XCellGuiError>;
