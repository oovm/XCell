use std::fmt::{Display, Formatter};

use super::*;

impl Display for XCellValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XCellValue::Boolean(v) => {
                write!(f, "{v}")
            }
            XCellValue::Integer8(v) => {
                write!(f, "{v}i8")
            }
            XCellValue::Integer16(v) => {
                write!(f, "{v}i16")
            }
            XCellValue::Integer32(v) => {
                write!(f, "{v}i32")
            }
            XCellValue::Integer64(v) => {
                write!(f, "{v}i64")
            }
            XCellValue::Unsigned8(v) => {
                write!(f, "{v}u8")
            }
            XCellValue::Unsigned16(v) => {
                write!(f, "{v}u16")
            }
            XCellValue::Unsigned32(v) => {
                write!(f, "{v}u32")
            }
            XCellValue::Unsigned64(v) => {
                write!(f, "{v}u64")
            }
            XCellValue::Float32(_) => {
                todo!()
            }
            XCellValue::Float64(_) => {
                todo!()
            }
            XCellValue::String(_) => {
                todo!()
            }
            XCellValue::Color(_) => {
                todo!()
            }
            XCellValue::Custom(_) => {
                todo!()
            }
        }
    }
}
