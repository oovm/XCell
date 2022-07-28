use super::*;
use crate::IntegerKind;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnumerateDescription {
    pub integer: IntegerKind,
    pub typing: String,
    pub default: String,
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
        Self { integer: Default::default(), typing: typing.into(), default: "".to_string() }
    }

    pub fn parse_cell(&self, cell: &DataType) -> XResult<String> {
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
