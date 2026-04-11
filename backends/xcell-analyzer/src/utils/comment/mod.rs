use super::*;
use calamine::DataType;

/// 代码注释类型，包含摘要和详细说明
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct XComment {
    /// 摘要内容
    pub summary: String,
    /// 详细说明
    pub detail: String,
}

/// C# 风格注释
pub struct CsComment {
    /// 摘要内容
    pub summary: String,
    /// 详细说明
    pub detail: String,
}

impl From<&Data> for XComment {
    fn from(value: &Data) -> Self {
        let mut out = XComment::default();
        if let Some(s) = value.get_string() {
            out.summary = s.to_string()
        }
        out
    }
}

impl XComment {
    /// 从行数据中读取文档注释
    pub fn read_document(row: &[Data], id: usize) -> Self {
        row.get(id).map(XComment::from).unwrap_or_default()
    }

    /// 从行数据中读取非零列的文档注释
    pub fn read_non_zero(row: &[Data], id: usize) -> Self {
        if id == 0 {
            return XComment::default();
        }
        row.get(id).map(XComment::from).unwrap_or_default()
    }

    /// 按行分割注释内容，生成 XML 标签格式
    pub fn lines(&self) -> Vec<String> {
        let mut out = String::new();
        if !self.summary.trim().is_empty() {
            out.push_str("<summary>");
            out.push('\n');
            out.push_str(&self.summary);
            out.push('\n');
            out.push_str("</summary>");
            out.push('\n');
        }
        if !self.detail.trim().is_empty() {
            out.push_str("<detail>");
            out.push('\n');
            out.push_str(&self.detail);
            out.push('\n');
            out.push_str("</detail>");
        }
        out.lines().map(|s| s.to_string()).collect()
    }
}
