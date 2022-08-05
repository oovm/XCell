use xcell_types::StringDescription;

use super::*;

mod enumerate;
mod number;
mod string;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XData {
    Number(Box<XDataNumber>),
    String(Box<XDataString>),
    Enumerate(Box<XDataEnumerate>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataNumber {
    pub headers: Vec<XCellHeader>,
    pub data: BTreeMap<BigInt, XDataItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataString {
    pub headers: Vec<XCellHeader>,
    pub data: BTreeMap<String, XDataItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataEnumerate {
    pub id_column: usize,
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
        self.key_item().map(|v| v.field_name).unwrap_or("id".into())
    }
    pub fn key_type(&self) -> XCellTyped {
        self.key_item().map(|v| v.typing).unwrap_or(StringDescription::default().into())
    }
    fn key_item(&self) -> Option<&XCellHeader> {
        match self {
            XData::Number(v) => v.headers.get(0),
            XData::String(v) => v.headers.get(0),
            XData::Enumerate(v) => v.headers.get(0),
        }
    }
    pub fn rows(&self) {}

    pub fn rows_count(&self) -> usize {
        match self {
            XData::Number(v) => v.data.len(),
            XData::String(v) => v.data.len(),
            XData::Enumerate(v) => v.data.len(),
        }
    }
}
