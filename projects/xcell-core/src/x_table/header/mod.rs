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
            XCellTyped::Boolean(b) => match b.parse_cell(cell) {
                Ok(o) => Ok(XCellValue::Boolean(o)),
                Err(e) => Err(XErrorKind::TypeMismatch { except: XCellTyped::Boolean(Default::default()), current: e }),
            },
            XCellTyped::Integer8 => {
                todo!()
            }
            XCellTyped::Integer16 => {
                todo!()
            }
            XCellTyped::Integer32 => {
                todo!()
            }
            XCellTyped::Integer64 => {
                todo!()
            }
            XCellTyped::Unsigned8 => {
                todo!()
            }
            XCellTyped::Unsigned16 => {
                todo!()
            }
            XCellTyped::Unsigned32 => {
                todo!()
            }
            XCellTyped::Unsigned64 => {
                todo!()
            }
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
            XCellTyped::Color => {
                todo!()
            }
            XCellTyped::Custom(_) => {
                todo!()
            }
        }
    }
}
