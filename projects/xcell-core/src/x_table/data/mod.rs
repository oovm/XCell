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
    pub data: BTreeMap<BigInt, XEnumerateItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataString {
    pub headers: Vec<XCellHeader>,
    pub data: BTreeMap<String, XEnumerateItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataEnumerate {
    pub id_column: usize,
    pub comment_column: usize,
    pub headers: Vec<XCellHeader>,
    pub data: BTreeMap<String, XEnumerateItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XEnumerateItem {
    pub name: String,
    pub id: Option<BigInt>,
    pub comment: String,
    pub data: Vec<XCellValue>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataNumberItem {
    pub key: BigInt,
    pub data: Vec<XCellValue>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XDataStringItem {
    pub key: String,
    pub data: Vec<XCellValue>,
}

impl XData {
    pub fn key_field(&self) -> String {
        let id = "id".to_string();
        match self {
            XData::Number(v) => v.headers.get(0).map(|v| v.field_name).unwrap_or(id),
            XData::String(v) => v.headers.get(0).map(|v| v.field_name).unwrap_or(id),
            XData::Enumerate(v) => v.headers.get(0).map(|v| v.field_name).unwrap_or(id),
        }
    }
    pub fn key_type(&self) -> String {
        let id = "id".to_string();
        match self {
            XData::Number(v) => v.headers.get(0).map(|v| v.field_name).unwrap_or(id),
            XData::String(v) => v.headers.get(0).map(|v| v.field_name).unwrap_or(id),
            XData::Enumerate(v) => v.headers.get(0).map(|v| v.field_name).unwrap_or(id),
        }
    }
    fn key_item(&self) -> Option<&XCellHeader> {
        match self {
            XData::Number(v) => v.headers.get(0),
            XData::String(v) => v.headers.get(0),
            XData::Enumerate(v) => v.headers.get(0),
        }
    }
}
