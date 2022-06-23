use std::io::Write;

use bytestream::{ByteOrder, StreamWriter};

use crate::XCellValue;

impl StreamWriter for XCellValue {
    fn write_to<W>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()>
    where
        W: Write,
    {
        match self {
            XCellValue::Boolean(_) => {
                todo!()
            }
            XCellValue::Integer8(v) => v.write_to(buffer, order),
            XCellValue::Integer16(v) => v.write_to(buffer, order),
            XCellValue::Integer32(v) => v.write_to(buffer, order),
            XCellValue::Integer64(v) => v.write_to(buffer, order),
            XCellValue::Unsigned8(v) => v.write_to(buffer, order),
            XCellValue::Unsigned16(v) => v.write_to(buffer, order),
            XCellValue::Unsigned32(v) => v.write_to(buffer, order),
            XCellValue::Unsigned64(v) => v.write_to(buffer, order),
            XCellValue::Float32(_) => {
                todo!()
            }
            XCellValue::Float64(_) => {
                todo!()
            }
            XCellValue::Vector2(_) => {
                todo!()
            }
            XCellValue::Vector3(_) => {
                todo!()
            }
            XCellValue::Vector4(_) => {
                todo!()
            }
            XCellValue::Color4(_) => {
                todo!()
            }
            XCellValue::Quaternion4(_) => {
                todo!()
            }
            XCellValue::String(_) => {
                todo!()
            }
            XCellValue::Color(_) => {
                todo!()
            }
            XCellValue::Time(_) => {
                todo!()
            }
            XCellValue::Custom(_) => {
                todo!()
            }
            XCellValue::Vector(_) => {
                todo!()
            }
        }
    }
}
