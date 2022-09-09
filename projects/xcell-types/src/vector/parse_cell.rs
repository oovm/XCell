use crate::utils::syntax_error;

use super::*;

impl From<VectorDescription> for XCellTyped {
    fn from(value: VectorDescription) -> Self {
        XCellTyped::Vector(Box::new(value))
    }
}

impl VectorDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let mut out = vec![];
        let s = match cell {
            DataType::Error(e) => return syntax_error(format!("未知错误 {e}")),
            _ => cell.to_string(),
        };
        if s.trim().is_empty() {
            return Ok(XCellValueKind::Vector(out));
        }
        for item in self.split(&s) {
            let cell = DataType::String(item.to_string());
            let s = self.typing.parse_cell(&cell)?;
            out.push(s)
        }
        Ok(XCellValueKind::Vector(out))
    }
    pub fn split<'i>(&self, s: &'i str) -> Vec<&'i str> {
        s.split(|c: char| self.delimiter.contains(&c)).map(|s| s.trim()).collect()
    }
}
