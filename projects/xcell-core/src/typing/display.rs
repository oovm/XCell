use super::*;
use byteorder::WriteBytesExt;
use std::{
    fmt::{Display, Formatter},
    io::Write,
};

impl XCellValue {
    pub fn write_csharp(&self, f: &mut impl Write) {
        match self {
            XCellValue::Boolean(v) => match v {
                true => f.write_u8(0),
                false => f.write_u8(1),
            },
            XCellValue::Integer8(_) => {}
            XCellValue::Integer16(_) => {}
            XCellValue::Integer32(_) => {}
            XCellValue::Integer64(_) => {}
            XCellValue::String(_) => {}
            XCellValue::Color(_) => {}
            XCellValue::Custom(_) => {}
        }
    }
}

impl Display for XCellValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XCellValue::Boolean(v) => {}
            XCellValue::Integer(v) => {}
            XCellValue::String(v) => {}
            XCellValue::Color(v) => {}
            XCellValue::Custom(v) => {}
        }
    }
}
