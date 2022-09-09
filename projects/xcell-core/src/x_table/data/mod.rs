use std::str::FromStr;

use itertools::Itertools;

use xcell_errors::Validation;
use xcell_types::{EnumerateDescription, IntegerKind, StringDescription, TypeMetaInfo};

use crate::{utils::first_not_nil, CalamineTable, Success, WorkspaceManager, XCellHeader, XCellTable};

use super::*;

pub use self::class::XClassItem;

mod class;
mod dictionary;
mod enumerate;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XData {
    Dictionary(Box<XDataDictionary>),
    Enumerate(Box<XDataEnumerate>),
    Class(Box<XDataClass>),
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDataClass {
    pub items: Vec<XClassItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataItem {
    pub id: BigInt,
    pub name: String,
    pub comment: String,
    pub data: Vec<XCellValue>,
}

impl XData {
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path, meta: &TypeMetaInfo) {
        match self {
            XData::Dictionary(v) => v.read_table_data(table, path),
            XData::Enumerate(v) => v.read_table_data(table, path),
            XData::Class(v) => v.read_table_data(table, path, meta),
        }
    }
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
            XData::Class(_) => None,
        }
    }
    pub fn rows(&self) -> Vec<&XDataItem> {
        match self {
            XData::Dictionary(v) => v.data.iter().collect(),
            XData::Enumerate(v) => v.data.values().collect(),
            XData::Class(_) => {
                vec![]
            }
        }
    }
    pub fn headers(&self) -> Vec<&XCellHeader> {
        match self {
            XData::Dictionary(v) => v.headers.iter().collect(),
            XData::Enumerate(v) => v.headers.iter().collect(),
            XData::Class(_) => {
                vec![]
            }
        }
    }
    pub fn rows_count(&self) -> usize {
        match self {
            XData::Dictionary(v) => v.data.len(),
            XData::Enumerate(v) => v.data.len(),
            XData::Class(v) => v.items.len(),
        }
    }
}
