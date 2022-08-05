use super::*;

impl XCellHeader {
    pub fn parse_cell(&self, row: &[DataType]) -> XResult<XCellValue> {
        match row.get(self.column) {
            Some(cell) => self.typing.parse_cell(cell),
            None => {
                todo!()
            }
        }
    }
}
