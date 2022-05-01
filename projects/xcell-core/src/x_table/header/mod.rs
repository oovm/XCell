use super::*;

impl XCellHeader {
    pub fn parse_cell(&self, cell: &DataType) {
        self.typing.parse_cell(cell)
    }
}
