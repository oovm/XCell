use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    Deserialize, Deserializer, Serialize,
    de::{MapAccess, Visitor},
};

use crate::{
    XResult,
    for_3rd::{Data, read_map_next_extra, read_map_next_value},
};

use crate::{XCellTyped, XCellValue, utils::syntax_error};

mod der;

#[derive(Debug, Clone, Serialize)]
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
        for pattern in &self.patterns {
            if s.eq_ignore_ascii_case(pattern) {
                return true;
            }
        }
        false
    }
    pub fn add_pattern(&mut self, s: impl Into<String>) {
        self.patterns.insert(s.into());
    }
    pub fn mut_pattern(&mut self) -> &mut BTreeSet<String> {
        &mut self.patterns
    }
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        self.parse_value(cell).map(XCellValue::String)
    }
    pub fn parse_value(&self, cell: &Data) -> XResult<String> {
        match cell {
            Data::Int(v) => Ok(v.to_string()),
            Data::Float(v) => Ok(v.to_string()),
            Data::String(v) => Ok(v.to_string()),
            Data::Bool(v) => Ok(v.to_string()),
            Data::DateTime(v) => Ok(v.to_string()),
            Data::Empty => Ok(self.default.clone()),
            Data::Error(e) => syntax_error(format!("未知错误 {e}")),
            Data::DateTimeIso(v) => Ok(v.to_string()),
            Data::DurationIso(v) => Ok(v.to_string()),
        }
    }
}
