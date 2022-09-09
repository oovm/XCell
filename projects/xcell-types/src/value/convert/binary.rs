use std::io::Write;

use stream_io::ByteOrder;

use crate::{StreamWriter, XCellValueKind};

impl StreamWriter for XCellValueKind {
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> std::io::Result<()> {
        match self {
            XCellValueKind::Boolean(v) => match v {
                true => 0u8.write_to(buffer, order)?,
                false => 1u8.write_to(buffer, order)?,
            },
            XCellValueKind::Integer8(v) => v.write_to(buffer, order)?,
            XCellValueKind::Integer16(v) => v.write_to(buffer, order)?,
            XCellValueKind::Integer32(v) => v.write_to(buffer, order)?,
            XCellValueKind::Integer64(v) => v.write_to(buffer, order)?,
            XCellValueKind::Unsigned8(v) => v.write_to(buffer, order)?,
            XCellValueKind::Unsigned16(v) => v.write_to(buffer, order)?,
            XCellValueKind::Unsigned32(v) => v.write_to(buffer, order)?,
            XCellValueKind::Unsigned64(v) => v.write_to(buffer, order)?,
            XCellValueKind::Float32(v) => v.write_to(buffer, order)?,
            XCellValueKind::Float64(v) => v.write_to(buffer, order)?,
            XCellValueKind::Vector2(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValueKind::Vector3(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValueKind::Vector4(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValueKind::Color4(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            XCellValueKind::Quaternion4(v) => {
                for item in v {
                    item.write_to(buffer, order)?;
                }
            }
            // https://en.wikipedia.org/wiki/Variable-length_quantity
            XCellValueKind::String(v) => {
                write_7_bit(v.len(), buffer, order)?;
                for item in v.bytes() {
                    item.write_to(buffer, order)?
                }
            }
            XCellValueKind::Color(v) => {
                v.r.write_to(buffer, order)?;
                v.g.write_to(buffer, order)?;
                v.b.write_to(buffer, order)?;
                v.a.write_to(buffer, order)?;
            }
            XCellValueKind::Vector(v) => {
                (v.len() as u32).write_to(buffer, order)?;
                for item in v.iter() {
                    item.write_to(buffer, order)?
                }
            }
            XCellValueKind::Enumerate(v) => {
                panic!("无法写入二进制 `{}`", v)
            }
        }
        Ok(())
    }
}

pub fn write_7_bit<W: Write>(length: usize, buffer: &mut W, order: ByteOrder) -> std::io::Result<()> {
    let mut value = length as u32;
    while value >= 0x80 {
        ((value | 0x80) as u8).write_to(buffer, order)?;
        value >>= 7;
    }
    (value as u8).write_to(buffer, order)
}
