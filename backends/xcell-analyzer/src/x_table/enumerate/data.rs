use std::path::PathBuf;

use crate::utils::comment::XComment;

use super::*;

/// 需要导出的枚举数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XEnumerateData {
    /// 该枚举的名称
    pub name: String,
    /// 该枚举的路径
    pub path: PathBuf,
    /// 该枚举的类型
    pub typing: IntegerDescription,
    /// 该枚举的注释
    pub comment: XComment,
    /// 该枚举的字段类型
    pub headers: Vec<XCellHeader>,
    /// 该枚举的字段值
    pub lines: Vec<XDataLine>,
}
