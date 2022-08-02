use xcell_errors::for_3rd::{Datelike, Timelike, Zero};

use crate::{ArrayDescription, ArrayKind, ColorDescription, DecimalDescription, DecimalKind, IntegerDescription, StringDescription, TimeDescription};

use super::*;

impl XCellTyped {
    pub fn as_csharp_default(&self) -> String {
        match self {
            XCellTyped::Boolean(v) => v.as_csharp_default(),
            XCellTyped::Integer(v) => v.as_csharp_default(),
            XCellTyped::Decimal(v) => v.as_csharp_default(),
            XCellTyped::String(v) => v.as_csharp_default(),
            XCellTyped::Time(v) => v.as_csharp_default(),
            XCellTyped::Color(v) => v.as_csharp_default(),
            XCellTyped::Enumerate(v) => v.default.to_string(),
            XCellTyped::Array(v) => v.as_csharp_default(),
            XCellTyped::Vector(_) => {
                todo!()
            }
        }
    }
}

impl BooleanDescription {
    pub fn as_csharp_default(&self) -> String {
        match self.default {
            true => "true".to_string(),
            false => "".to_string(),
        }
    }
}

impl IntegerDescription {
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_zero() { "".to_string() } else { self.default.to_string() }
    }
}

impl DecimalDescription {
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_zero() { "".to_string() } else { self.default.to_string() }
    }
}

impl StringDescription {
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_empty() { "".to_string() } else { format!("{:?}", self.default) }
    }
}

impl TimeDescription {
    fn as_csharp_default(&self) -> String {
        match &self.default {
            Some(s) => {
                format!(
                    "new DateTime({year}, {month}, {day}, {hour}, {minute}, {second})",
                    year = s.year(),
                    month = s.month(),
                    day = s.day(),
                    hour = s.hour(),
                    minute = s.minute(),
                    second = s.second()
                )
            }
            None => {
                format!("new DateTime()")
            }
        }
    }
}

impl ColorDescription {
    fn as_csharp_default(&self) -> String {
        let [r, g, b, a] = self.default.to_rgba8();
        format!("new Color32({r}, {g}, {b}, {a})")
    }
}

impl ArrayDescription {
    fn as_csharp_default(&self) -> String {
        // match self.kind {
        //     ArrayKind::Vector2 => {}
        //     ArrayKind::Vector3 => {}
        //     ArrayKind::Vector4 => {}
        //     ArrayKind::Color4 => {}
        //     ArrayKind::Quaternion4 => {}
        // }
        //
        // let [r, g, b, a] = self.default.to_rgba8();
        format!("new ()")
    }
}

impl XCellTyped {
    pub fn make_cs_binary_writer(&self, field: &str) -> Vec<String> {
        let mut out = vec![];
        match self {
            XCellTyped::Time(_) => {
                out.push(format!("w.Write({field}.Ticks);"));
            }
            XCellTyped::Color(_) => {
                out.push(format!("w.Write({field}.r);"));
                out.push(format!("w.Write({field}.g);"));
                out.push(format!("w.Write({field}.b);"));
                out.push(format!("w.Write({field}.a);"));
            }
            XCellTyped::Enumerate(v) => out.push(v.default.to_string()),
            _ => {
                out.push(format!("w.Write({field});"));
            }
        }
        out
    }
    pub fn make_cs_binary_reader(&self, field: &str) -> Vec<String> {
        match self {
            XCellTyped::Boolean(v) => vec![v.as_csharp_reader(field)],
            XCellTyped::Integer(v) => vec![v.as_csharp_reader(field)],
            XCellTyped::Decimal(v) => vec![v.as_csharp_reader(field)],
            XCellTyped::String(v) => "r.ReadString()".to_string(),
            XCellTyped::Time(_) => "new DateTime(r.ReadInt64(), DateTimeKind.Utc)".to_string(),
            XCellTyped::Color(_) => "new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())".to_string(),
            XCellTyped::Enumerate(v) => v.as_csharp_reader(),
            XCellTyped::Array(_) => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())".to_string(),
            XCellTyped::Vector(v) => {
                r#"
var skillIdsCount = r.ReadUInt32();
skillIds = new List<string>((int) skillIdsCount);
for (var i = 0; i < skillIdsCount; i++)
{
    skillIds.Add(r.ReadString());
}
                
                "#

                v.typing.make_cs_binary_reader()

            }
        }
    }
    pub fn as_csharp_reader_function(&self, field: &str) -> String {
        let str = match self {
            XCellTyped::Boolean(_) => {"r.ReadBoolean()"}
            XCellTyped::Integer(v) => {
                v.kind.as_csharp_reader()
            }
            XCellTyped::Decimal(v) => { match v.kind {
                DecimalKind::Float32 => "r.ReadSingle()",
                DecimalKind::Float64 => "r.ReadDouble()",
                DecimalKind::Decimal128 => "r.ReadDecimal()",
            } }
            XCellTyped::String(_) => {"r.ReadString()"}
            XCellTyped::Time(_) => {"new DateTime(r.ReadInt64(), DateTimeKind.Utc)"}
            XCellTyped::Color(_) => {"new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"}
            XCellTyped::Enumerate(v) => {
                return format!("({}) {}", field, v.integer.as_csharp_reader())
            }
            XCellTyped::Array(v) => {
                match v.kind {
                    ArrayKind::Vector2 => {"new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"}
                    ArrayKind::Vector3 => {"new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"}
                    ArrayKind::Vector4 => {"new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"}
                    ArrayKind::Color4 => unreachable!(),
                    ArrayKind::Quaternion4 => {"new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"}
                }

            }
            XCellTyped::Vector(_) => unreachable!()
        };
        str.to_string()
    }
}

impl BooleanDescription {
    pub fn as_csharp_reader(&self, field: &str) -> String {
        format!("{field} = r.{reader}()", reader = self.as_csharp_reader_function())
    }
    pub fn as_csharp_reader_function(&self) -> &'static str {
        "ReadBoolean"
    }
}

impl IntegerKind {
    pub fn as_csharp_type(&self) -> &'static str {
        match self {
            IntegerKind::Integer8 => "byte",
            IntegerKind::Integer16 => "short",
            IntegerKind::Integer32 => "int",
            IntegerKind::Integer64 => "long",
            IntegerKind::Unsigned8 => "sbyte",
            IntegerKind::Unsigned16 => "ushort",
            IntegerKind::Unsigned32 => "uint",
            IntegerKind::Unsigned64 => "ulong",
        }
    }
    pub fn as_csharp_reader(&self) -> &'static str {
        match self {
            IntegerKind::Integer8 => "r.ReadByte()",
            IntegerKind::Integer16 => "r.ReadInt16()",
            IntegerKind::Integer32 => "r.ReadInt32()",
            IntegerKind::Integer64 => "r.ReadInt64()",
            IntegerKind::Unsigned8 => "r.ReadSByte()",
            IntegerKind::Unsigned16 => "r.ReadUInt16()",
            IntegerKind::Unsigned32 => "r.ReadUInt32()",
            IntegerKind::Unsigned64 => "r.ReadUInt64()",
        }
    }
}

impl IntegerDescription {
    pub fn as_csharp_reader(&self, field: &str) -> String {
        format!("{field} = r.{reader}()", reader = self.kind.as_csharp_reader_function())
    }
}

impl DecimalKind {
    pub fn as_csharp_type(&self) -> &'static str {
        match self {
            DecimalKind::Float32 => "float",
            DecimalKind::Float64 => "double",
            DecimalKind::Decimal128 => "decimal",
        }
    }
}

impl DecimalDescription {
    pub fn as_csharp_reader(&self, field: &str) -> String {
        format!("{field} = r.{reader}()", reader = self.kind.as_csharp_reader())
    }
}

impl StringDescription {
    pub fn as_csharp_reader(&self) -> String {
        format!("({}) r.{}()", self.typing, self.integer.as_csharp_reader_function())
    }
}

