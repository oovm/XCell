use crate::errors::syntax_error;

use super::*;

impl VectorDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {
        let mut out = vec![];
        let s = match cell {
            DataType::Error(e) => return syntax_error(format!("未知错误 {e}")),
            _ => cell.to_string(),
        };
        for item in s.split(',').map(|s| s.trim()) {
            let cell = DataType::String(item.to_string());
            let s = self.typing.parse_cell(&cell)?;
            out.push(s)
        }
        Ok(XCellValue::Vector(out))
    }
}
