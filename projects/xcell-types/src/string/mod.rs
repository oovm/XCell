use crate::{utils::syntax_error, XCellTyped, XCellValue};
use serde::{Deserialize, Serialize};
use xcell_errors::{for_3rd::DataType, XResult};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StringDescription {
    pub default: String,
}

impl From<StringDescription> for XCellTyped {
    fn from(value: StringDescription) -> Self {
        Self::String(Box::new(value))
    }
}

impl StringDescription {
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
