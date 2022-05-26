use num::ToPrimitive;

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
            XCellTyped::Integer8(typing) => typing.parse_i8(cell).map(|v| XCellValue::Integer8(v)),
            XCellTyped::Integer16(typing) => typing.parse_i16(cell).map(|v| XCellValue::Integer16(v)),
            XCellTyped::Integer32(typing) => typing.parse_i32(cell).map(|v| XCellValue::Integer32(v)),
            XCellTyped::Integer64(typing) => typing.parse_i64(cell).map(|v| XCellValue::Integer64(v)),
            XCellTyped::Unsigned8(typing) => typing.parse_u8(cell).map(|v| XCellValue::Unsigned8(v)),
            XCellTyped::Unsigned16(typing) => typing.parse_u16(cell).map(|v| XCellValue::Unsigned16(v)),
            XCellTyped::Unsigned32(typing) => typing.parse_u32(cell).map(|v| XCellValue::Unsigned32(v)),
            XCellTyped::Unsigned64(typing) => typing.parse_u64(cell).map(|v| XCellValue::Unsigned64(v)),
            XCellTyped::Float32(typing) => typing.parse_f32(cell).map(|v| XCellValue::Float32(v)),
            XCellTyped::Float64(typing) => typing.parse_f64(cell).map(|v| XCellValue::Float64(v)),
            XCellTyped::Decimal128(typing) => typing.parse_f64(cell).map(|v| XCellValue::Float64(v)),
            XCellTyped::String(typing) => typing.parse_cell(cell).map(|v| XCellValue::String(v)),
            XCellTyped::LanguageID(typing) => typing.parse_cell(cell).map(|v| XCellValue::String(v)),
            XCellTyped::Datetime(typing) => {
                typing.parse_cell(cell).map(|v| XCellValue::Integer8(v.to_i8().unwrap_or_default()))
            }
            XCellTyped::Color(typing) => typing.parse_cell(cell).map(|v| XCellValue::Color(v)),
            XCellTyped::Custom(typing) => typing.parse_cell(cell).map(|v| XCellValue::String(v)),
        }
    }
}
