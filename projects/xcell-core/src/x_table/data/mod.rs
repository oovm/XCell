use super::*;

mod enumerate;
mod string;
mod number;

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
