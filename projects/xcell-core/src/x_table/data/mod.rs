use crate::{x_table::data::class::XClassItem, XEnumerateData, XLanguageTable};

use super::*;

mod class;
mod dictionary;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XTableKind {
    /// 不需要导出的数据
    Internal,
    Array(Box<XArrayTable>),
    Dictionary(Box<XDictionaryTable>),
    Enumerate(Box<XEnumerateData>),
    Class(Box<XClassTable>),
    Language(Box<XLanguageTable>),
}

impl Default for XTableKind {
    fn default() -> Self {
        Self::Array(Default::default())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XArrayTable {
    pub headers: Vec<XCellHeader>,
    pub data: Vec<XDataItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDictionaryTable {
    pub headers: Vec<XCellHeader>,
    pub data: Vec<XDataItem>,
}

/// 生成一个类, 适用于全局配置
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XClassTable {
    pub items: Vec<XClassItem>,
    pub data: Vec<XCellValue>,
}

/// 字段数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataItem {
    pub id: BigInt,
    /// 该字段的名称
    pub name: String,
    pub comment: String,
    pub data: Vec<XCellValue>,
}

impl XTableKind {
    pub fn read_table_data(&mut self, table: &CalamineTable3, path: &Path, meta: &TypeMetaInfo) {
        match self {
            XTableKind::Array(v) => v.read_table_data(table, path),
            XTableKind::Enumerate(v) => v.read_table_data(table, path),
            XTableKind::Class(v) => v.read_table_data(table, path, meta),
            XTableKind::Dictionary(_) => {
                todo!()
            }
            XTableKind::Language(_) => {}
        }
    }
}

impl XTableKind {
    pub fn key_field(&self) -> String {
        self.key_item().map(|v| v.field_name.as_str()).unwrap_or("id").to_string()
    }
    pub fn key_type(&self) -> XCellTyped {
        self.key_item().map(|v| v.typing.clone()).unwrap_or_else(|| StringDescription::default().into())
    }
    fn key_item(&self) -> Option<&XCellHeader> {
        match self {
            XTableKind::Array(v) => v.headers.get(0),
            XTableKind::Enumerate(v) => v.headers.get(0),
            XTableKind::Class(_) => None,
            XTableKind::Dictionary(_) => {
                todo!()
            }
            XTableKind::Language(_) => {
                todo!()
            }
        }
    }
    pub fn rows(&self) -> Vec<&XDataItem> {
        match self {
            XTableKind::Array(v) => v.data.iter().collect(),
            XTableKind::Enumerate(v) => v.data.values().collect(),
            XTableKind::Class(_) => vec![],
            XTableKind::Dictionary(_) => {
                todo!()
            }
            XTableKind::Language(_) => {
                todo!()
            }
        }
    }
    pub fn headers(&self) -> Vec<&XCellHeader> {
        match self {
            XTableKind::Array(v) => v.headers.iter().collect(),
            XTableKind::Enumerate(v) => v.headers.iter().collect(),
            XTableKind::Class(_) => {
                vec![]
            }
            XTableKind::Dictionary(_) => {
                todo!()
            }
            XTableKind::Language(_) => {
                todo!()
            }
        }
    }
    pub fn rows_count(&self) -> usize {
        match self {
            XTableKind::Array(v) => v.data.len(),
            XTableKind::Enumerate(v) => v.data.len(),
            XTableKind::Class(v) => v.items.len(),
            XTableKind::Dictionary(_) => {
                todo!()
            }
            XTableKind::Language(_) => {
                todo!()
            }
        }
    }
}
