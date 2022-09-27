use crate::{
    x_table::{class::XClassData, dictionary::XDictData},
    XListData,
};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XExportData {
    /// 不需要导出的数据
    Internal,
    /// key 为数字的表
    List(Box<XListData>),
    /// key 为字符串的表
    Dict(Box<XDictData>),
    /// 类定义
    Class(Box<XClassData>),
    /// 枚举定义
    Enumerate(Box<XEnumerateData>),
    // Language(Box<XLanguageTable>),
}

impl Default for XExportData {
    fn default() -> Self {
        Self::Internal
    }
}
