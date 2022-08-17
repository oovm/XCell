use crate::utils::syntax_error;

use super::*;

impl From<VectorDescription> for XCellTyped {
    fn from(value: VectorDescription) -> Self {
        XCellTyped::Vector(Box::new(value))
    }
}

impl VectorDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {
        let mut out = vec![];
        let s = match cell {
            DataType::Error(e) => return syntax_error(format!("未知错误 {e}")),
            _ => cell.to_string(),
        };
        for item in self.split(&s) {
            let cell = DataType::String(item.to_string());
            let s = self.typing.parse_cell(&cell)?;
            out.push(s)
        }
        Ok(XCellValue::Vector(out))
    }
    pub fn split(&self, s: &str) -> Vec<&str> {
        s.split(|c: char| self.delimiter.contains(&c)).map(|s| s.trim()).collect()
    }
}
