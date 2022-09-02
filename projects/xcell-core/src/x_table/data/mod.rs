use crate::XCellHeader;
use xcell_types::{IntegerKind, StringDescription};

use super::*;

mod dictionary;
mod enumerate;
mod string;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XData {
    Dictionary(Box<XDataDictionary>),
    Enumerate(Box<XDataEnumerate>),
}

impl Default for XData {
    fn default() -> Self {
        Self::Dictionary(Default::default())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDataDictionary {
    pub headers: Vec<XCellHeader>,
    pub data: Vec<XDataItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDataEnumerate {
    /// 0 表示未设置
    pub id_column: usize,
    pub id_type: IntegerKind,
    pub comment_column: usize,
    pub headers: Vec<XCellHeader>,
    pub data: BTreeMap<String, XDataItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataItem {
    pub id: BigInt,
    pub name: String,
    pub comment: String,
    pub data: Vec<XCellValue>,
}

impl XData {
    pub fn key_field(&self) -> String {
        self.key_item().map(|v| v.field_name.as_str()).unwrap_or("id").to_string()
    }
    pub fn key_type(&self) -> XCellTyped {
        self.key_item().map(|v| v.typing.clone()).unwrap_or_else(|| StringDescription::default().into())
    }
    fn key_item(&self) -> Option<&XCellHeader> {
        match self {
            XData::Dictionary(v) => v.headers.get(0),
            XData::Enumerate(v) => v.headers.get(0),
        }
    }
    pub fn rows(&self) -> Vec<&XDataItem> {
        match self {
            XData::Dictionary(v) => v.data.iter().collect(),
            XData::Enumerate(v) => v.data.values().collect(),
        }
    }
    pub fn headers(&self) -> Vec<&XCellHeader> {
        match self {
            XData::Dictionary(v) => v.headers.iter().collect(),
            XData::Enumerate(v) => v.headers.iter().collect(),
        }
    }
    pub fn rows_count(&self) -> usize {
        match self {
            XData::Dictionary(v) => v.data.len(),
            XData::Enumerate(v) => v.data.len(),
        }
    }
}
