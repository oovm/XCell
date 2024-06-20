//! XCell GUI 错误模块
//!
//! 定义应用错误类型和处理

/// XCell GUI 错误类型
#[derive(Debug, thiserror::Error)]
pub enum XCellGuiError {
    /// 通用错误
    #[error("General error: {0}")]
    General(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, XCellGuiError>;
