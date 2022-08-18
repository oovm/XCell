use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use xcell_errors::{
    for_3rd::{read_map_next_extra, read_map_next_value, DataType},
    XResult,
};

use crate::{utils::syntax_error, XCellTyped, XCellValue};

mod der;

#[derive(Debug, Default, Clone, Serialize)]
pub struct StringDescription {
    patterns: BTreeSet<String>,
    pub default: String,
}

impl From<StringDescription> for XCellTyped {
    fn from(value: StringDescription) -> Self {
        Self::String(Box::new(value))
    }
}

impl StringDescription {
    pub fn matches_type(&self, s: &str) -> bool {
        self.patterns.contains(s)
    }
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {
        self.parse_value(cell).map(XCellValue::String)
    }
    pub fn parse_value(&self, cell: &DataType) -> XResult<String> {
        match cell {
            DataType::Int(v) => Ok(v.to_string()),
            DataType::Float(v) => Ok(v.to_string()),
            DataType::String(v) => Ok(v.to_string()),
            DataType::Bool(v) => Ok(v.to_string()),
            DataType::DateTime(v) => Ok(v.to_string()),
            DataType::Empty => Ok(self.default.clone()),
            DataType::Error(e) => syntax_error(format!("未知错误 {e}")),
        }
    }
}
