
use super::*;

/// 需要导出的枚举数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XEnumerateData {
    /// 该枚举的名称
    pub name: String,
    /// 该枚举的注释
    pub comment: XDocument,
    /// 该枚举的字段
    pub data: Vec<XDataItem>,
}


