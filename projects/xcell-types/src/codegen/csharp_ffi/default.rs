use xcell_errors::for_3rd::{Datelike, Timelike, Zero};

use crate::{ArrayDescription, ColorDescription, DecimalDescription, IntegerDescription, StringDescription, TimeDescription};

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
            XCellTyped::Custom(v) => v.default.to_string(),
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
    pub fn make_cs_binary_reader(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "r.ReadBoolean()".to_string(),
            XCellTyped::Integer(v) => format!("r.{}()", v.as_csharp_reader()),
            XCellTyped::Decimal(v) => format!("r.{}()", v.as_csharp_reader()),
            XCellTyped::String(_) => "r.ReadString()".to_string(),
            XCellTyped::Time(_) => "new DateTime(r.ReadInt64(), DateTimeKind.Utc)".to_string(),
            XCellTyped::Color(_) => "new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())".to_string(),
            XCellTyped::Enumerate(v) => v.default.to_string(),
            XCellTyped::Custom(v) => v.default.to_string(),
            XCellTyped::Array(_) => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())".to_string(),
            XCellTyped::Vector(_) => {
                todo!()
            }
        }
    }
}
