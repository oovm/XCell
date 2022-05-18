use crate::{typing::XCellValue, XErrorKind};

use super::*;

impl XCellHeader {
    pub fn parse_cell(&self, row: &[DataType]) -> Result<XCellValue, XErrorKind> {
        match row.get(self.column) {
            Some(cell) => self.typing.parse_cell(cell),
            None => {
                todo!()
            }
        }
    }
}

impl XCellTyped {
    pub fn parse_cell(&self, cell: &DataType) -> Result<XCellValue, XErrorKind> {
        match self {
            XCellTyped::Boolean(typing) => typing.parse_cell(cell).map(|v| XCellValue::Boolean(v)),
            XCellTyped::Integer8(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Integer16(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Integer32(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Integer64(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Unsigned8(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Unsigned16(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Unsigned32(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Unsigned64(typing) => typing.parse_cell(cell).map(|v| XCellValue::Integer(v)),
            XCellTyped::Float32 => {
                todo!()
            }
            XCellTyped::Float64 => {
                todo!()
            }
            XCellTyped::Float128 => {
                todo!()
            }
            XCellTyped::String => {
                todo!()
            }
            XCellTyped::LanguageID => {
                todo!()
            }
            XCellTyped::Datetime => {
                todo!()
            }
            XCellTyped::Color(typing) => typing.parse_cell(cell).map(|v| XCellValue::Color(v)),
            XCellTyped::Custom(_) => {
                todo!()
            }
        }
    }
}
