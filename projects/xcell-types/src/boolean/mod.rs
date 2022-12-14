use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use serde_types::OneOrMany;
use xcell_errors::{
    for_3rd::{read_map_next_extra, read_map_next_value, DataType},
    XResult,
};

use crate::utils::{syntax_error, type_mismatch};

use super::*;

mod der;

#[derive(Debug, Clone, Serialize)]
pub struct BooleanDescription {
    pub accept: BTreeSet<String>,
    pub reject: BTreeSet<String>,
    pub default: bool,
}

impl From<BooleanDescription> for XCellTyped {
    fn from(value: BooleanDescription) -> Self {
        Self::Boolean(Box::new(value))
    }
}

impl BooleanDescription {
    pub fn new(default: bool) -> BooleanDescription {
        Self { default, ..Self::default() }
    }

    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValueKind> {
        self.parse_value(cell).map(XCellValueKind::Boolean)
    }
    fn parse_value(&self, cell: &DataType) -> XResult<bool> {
        match cell {
            DataType::String(s) => {
                if self.accept.contains(s.to_ascii_lowercase().trim()) {
                    Ok(true)
                }
                else if self.reject.contains(s.to_ascii_lowercase().trim()) {
                    Ok(false)
                }
                else {
                    syntax_error(format!("{} 无法解析为 bool 类型", s))
                }
            }
            DataType::Bool(v) => Ok(*v),
            DataType::Empty => Ok(self.default),
            DataType::Error(e) => syntax_error(format!("未知错误 {e}")),
            _ => type_mismatch("Boolean", cell),
        }
    }
}
