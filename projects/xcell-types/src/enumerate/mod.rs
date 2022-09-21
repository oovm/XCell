use crate::{utils::syntax_error, IntegerKind, XCellTyped, XCellValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use xcell_errors::{
    for_3rd::{BigInt, DataType},
    XResult,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnumerateDescription {
    /// 枚举的名称
    pub name: String,
    /// 枚举的类型
    pub integer: IntegerKind,
    /// 枚举的默认值
    pub default: String,
    /// 枚举的字段映射
    pub mapping: BTreeMap<String, BigInt>,
}

impl From<EnumerateDescription> for XCellTyped {
    fn from(value: EnumerateDescription) -> Self {
        Self::Enumerate(Box::new(value))
    }
}

impl EnumerateDescription {
    pub fn new<S>(typing: S) -> Self
    where
        S: Into<String>,
    {
        Self { integer: Default::default(), name: typing.into(), default: "".to_string(), mapping: Default::default() }
    }
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {
        self.parse_value(cell).map(XCellValue::Enumerate)
    }
    fn parse_value(&self, cell: &DataType) -> XResult<String> {
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

impl XCellTyped {
    pub fn as_enumerate(&self) -> Option<&EnumerateDescription> {
        match self {
            XCellTyped::Enumerate(e) => Some(e),
            _ => None,
        }
    }
    pub fn mut_enumerate(&mut self) -> Option<&mut EnumerateDescription> {
        match self {
            XCellTyped::Enumerate(e) => Some(e),
            _ => None,
        }
    }
    pub fn is_enumerate(&self) -> bool {
        self.as_enumerate().is_some()
    }
}
