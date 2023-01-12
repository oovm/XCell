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

impl XExportData {
    pub fn key_field(&self) -> String {
        self.key_item().map(|v| v.field_name.as_str()).unwrap_or("id").to_string()
    }
    pub fn key_type(&self) -> XCellTyped {
        self.key_item().map(|v| v.typing.clone()).unwrap_or_else(|| StringDescription::default().into())
    }
    fn key_item(&self) -> Option<&XCellHeader> {
        match self {}
    }
    pub fn rows(&self) -> Vec<&XDataItem> {
        match self {
            XExportData::List(v) => v.data.iter().collect(),
            XExportData::Enumerate(v) => v.data.values().collect(),
            XExportData::Class(_) => vec![],
            XExportData::Dict(_) => {
                todo!()
            }
            XExportData::Language(_) => {
                todo!()
            }
        }
    }
    pub fn headers(&self) -> Vec<&XCellHeader> {
        match self {
            XExportData::List(v) => v.headers.iter().collect(),
            XExportData::Enumerate(v) => v.headers.iter().collect(),
            XExportData::Class(_) => {
                vec![]
            }
            XExportData::Dict(_) => {
                todo!()
            }
            XExportData::Language(_) => {
                todo!()
            }
        }
    }
    pub fn rows_count(&self) -> usize {
        match self {
            XExportData::List(v) => v.data.len(),
            XExportData::Enumerate(v) => v.data.len(),
            XExportData::Class(v) => v.items.len(),
            XExportData::Dict(_) => {
                todo!()
            }
            XExportData::Language(_) => {
                todo!()
            }
        }
    }
}
