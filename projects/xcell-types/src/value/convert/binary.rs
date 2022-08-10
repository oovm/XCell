use std::io::Write;

use stream_io::ByteOrder;

use crate::{StreamWriter, XCellValue};

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
            // https://en.wikipedia.org/wiki/Variable-length_quantity
            // https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-spptc/1eeaf7cc-f60b-4144-aa12-4eb9f6e748d1
            XCellValue::String(v) => {
                let mut value = v.len() as u32;
                while value >= 0x80 {
                    ((value | 0x80) as u8).write_to(buffer, order)?;
                    value >>= 7;
                }
                (value as u8).write_to(buffer, order)?;
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
            XCellValue::Vector(v) => {
                (v.len() as u32).write_to(buffer, order)?;
                for item in v.iter() {
                    item.write_to(buffer, order)?
                }
            }
            XCellValue::Enumerate(v) => {
                panic!("无法写入二进制 `{}`", v)
            }
        }
        Ok(())
    }
}
