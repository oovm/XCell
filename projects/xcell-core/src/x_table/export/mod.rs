use super::*;
use crate::{x_table::class::XClassData, XArrayData, XDictionaryTable};
use crate::x_table::dictionary::XDictionaryData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XExportData {
    /// 不需要导出的数据
    Internal,
    /// key 为数字的表
    Array(Box<XArrayData>),
    /// key 为字符串的表
    Dictionary(Box<XDictionaryData>),
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

/// 字段数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataItem {
    pub id: BigInt,
    /// 该字段的名称
    pub name: String,
    pub comment: XDocument,
    pub data: Vec<XCellValue>,
}

impl XExportData {
    pub fn read_table_data(&mut self, table: &CalamineTable3, path: &Path, meta: &TypeMetaInfo) {
        match self {
            XExportData::Enumerate(v) => v.read_table_data(table, path),
            XExportData::Internal => {}
        }
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
        match self {
            XExportData::Internal => {}
            XExportData::Enumerate(v) => v.headers.get(0),
        }
    }
    pub fn rows(&self) -> Vec<&XDataItem> {
        match self {
            XExportData::Array(v) => v.data.iter().collect(),
            XExportData::Enumerate(v) => v.data.values().collect(),
            XExportData::Class(_) => vec![],
            XExportData::Dictionary(_) => {
                todo!()
            }
            XExportData::Language(_) => {
                todo!()
            }
        }
    }
    pub fn headers(&self) -> Vec<&XCellHeader> {
        match self {
            XExportData::Array(v) => v.headers.iter().collect(),
            XExportData::Enumerate(v) => v.headers.iter().collect(),
            XExportData::Class(_) => {
                vec![]
            }
            XExportData::Dictionary(_) => {
                todo!()
            }
            XExportData::Language(_) => {
                todo!()
            }
        }
    }
    pub fn rows_count(&self) -> usize {
        match self {
            XExportData::Array(v) => v.data.len(),
            XExportData::Enumerate(v) => v.data.len(),
            XExportData::Class(v) => v.items.len(),
            XExportData::Dictionary(_) => {
                todo!()
            }
            XExportData::Language(_) => {
                todo!()
            }
        }
    }
}
