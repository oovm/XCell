use serde::{Deserialize, Serialize};

/// 文档类型
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDocument {
    /// 文档内容
    content: String,
}

impl XDocument {
    /// 创建新的文档
    pub fn new(content: impl Into<String>) -> Self {
        Self { content: content.into() }
    }

    /// 获取文档内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 按行分割文档内容
    pub fn lines(&self) -> Vec<String> {
        self.content.lines().map(|s| s.to_string()).collect()
    }

    /// 检查文档是否为空
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl From<String> for XDocument {
    fn from(content: String) -> Self {
        Self { content }
    }
}

impl From<&str> for XDocument {
    fn from(content: &str) -> Self {
        Self { content: content.to_string() }
    }
}
