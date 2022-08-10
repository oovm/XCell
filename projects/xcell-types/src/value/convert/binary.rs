use crate::{StreamWriter, XCellValue};
use integer_encoding::*;
use std::io::Write;
use stream_io::ByteOrder;

impl StreamWriter for XCellValue {
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()> {
        match self {
            XCellValue::Boolean(v) => match v {
                true => 0u8.write_to(buffer, order)?,
                false => 1u8.write_to(buffer, order)?,
            },
            XCellValue::Integer8(v) => v.write_to(buffer, order)?,
            XCellValue::Integer16(v) => v.write_to(buffer, order)?,
            XCellValue::Integer32(v) => v.write_to(buffer, order)?,
            XCellValue::Integer64(v) => v.write_to(buffer, order)?,
            XCellValue::Unsigned8(v) => v.write_to(buffer, order)?,
            XCellValue::Unsigned16(v) => v.write_to(buffer, order)?,
            XCellValue::Unsigned32(v) => v.write_to(buffer, order)?,
            XCellValue::Unsigned64(v) => v.write_to(buffer, order)?,
            XCellValue::Float32(v) => v.write_to(buffer, order)?,
            XCellValue::Float64(v) => v.write_to(buffer, order)?,
            XCellValue::Vector2(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValue::Vector3(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValue::Vector4(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValue::Color4(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValue::Quaternion4(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValue::String(v) => {
                for byte in (v.len() as i32).encode_var_vec() {
                    byte.write_to(buffer, order)?
                }
                for item in v.bytes() {
                    item.write_to(buffer, order)?
                }
            }
            XCellValue::Color(v) => {
                v.r.write_to(buffer, order)?;
                v.g.write_to(buffer, order)?;
                v.b.write_to(buffer, order)?;
                v.a.write_to(buffer, order)?;
            }
            XCellValue::Custom(_) => {
                todo!()
            }
            XCellValue::Vector(v) => {
                (v.len() as u32).write_to(buffer, order)?;
                for item in v.iter() {
                    item.write_to(buffer, order)?
                }
            }
        }
        Ok(())
    }
}
