use crate::typing::{IntegerKind, TimeDescription};
use chrono::{Datelike, Timelike};

use super::*;

impl UnityCodegen {
    pub fn write_class(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        tera_render(include_str!("PartClass.cs"), &self.make_context(table), path)?;
        Ok(())
    }
    pub fn write_interface(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        tera_render(include_str!("PartInterface.cs"), &self.make_context(table), path)?;
        Ok(())
    }
}

impl UnityCodegen {
    fn make_context(&self, table: &XCellTable) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("config", &self);
        ctx.insert("CLASS_NAME", &table.class_name());
        ctx.insert("TABLE_NAME", &format!("{}{}", table.class_name(), self.suffix_table));
        ctx.insert("ELEMENT_NAME", &format!("{}{}", table.class_name(), self.suffix_element));
        ctx.insert("ELEMENT_GETTER", &format!("Get{}", self.suffix_element));
        ctx.insert("ID_TYPE", &table.headers.key_type());
        let is_enum = table.is_enumerate();
        ctx.insert("enumerate", &is_enum);
        ctx.insert("CLASS_FIELDS", &table.headers.make_class_field(is_enum));
        ctx
    }
}

#[derive(Serialize)]
struct CsField {
    summary: Vec<String>,
    remarks: Vec<String>,
    writer: Vec<String>,
    typing: String,
    reader: String,
    name: String,
    getter: String,
    default: String,
    has_default: bool,
}

impl XCellHeaders {
    fn make_class_field(&self, is_enum: bool) -> Vec<CsField> {
        let mut items = self.inner.iter();
        if is_enum {
            items.next();
        }
        items.map(|v| v.make_class_field(is_enum)).collect()
    }
    fn key_type(&self) -> String {
        match self.inner.first() {
            Some(s) => match &s.typing {
                XCellTyped::Enumerate(e) => e.integer.as_csharp_type().to_string(),
                _ => s.typing.as_csharp_type(),
            },
            None => "int".to_string(),
        }
    }
}

impl XCellHeader {
    fn make_class_field(&self, _is_enum: bool) -> CsField {
        let default = self.typing.make_cs_default();
        CsField {
            summary: self.summary.lines().map(|v| v.to_string()).collect(),
            remarks: self.details.lines().map(|v| v.to_string()).collect(),
            has_default: !default.is_empty(),
            typing: self.typing.as_csharp_type(),
            writer: self.typing.make_cs_binary_writer(&self.field_name),
            reader: self.typing.make_cs_binary_reader(),
            name: self.field_name.clone(),
            getter: format!("Get{}", self.field_name.to_case(Case::Pascal)),
            default,
        }
    }
}

impl XCellTyped {
    pub fn as_csharp_type(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "bool".to_string(),
            XCellTyped::Integer(v) => v.as_csharp_type().to_string(),
            XCellTyped::Decimal(v) => v.as_csharp_type().to_string(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Time(_) => "DateTime".to_string(),
            XCellTyped::Color(_) => "Color32".to_string(),
            XCellTyped::Array(_) => {}
            XCellTyped::Enumerate(v) => v.typing.to_owned(),
            XCellTyped::Custom(v) => v.typing.to_owned(),
        }
    }
    pub fn make_cs_default(&self) -> String {
        match self {
            XCellTyped::Boolean(v) => v.default.to_string(),
            XCellTyped::Integer(v) => v.default.to_string(),
            XCellTyped::Decimal(v) => v.default.to_string(),
            XCellTyped::String(v) => format!("{:?}", v.default),
            XCellTyped::Time(v) => v.make_cs_datetime(),
            XCellTyped::Color(v) => v.make_cs_color32(),
            XCellTyped::Enumerate(v) => v.default.to_string(),
            XCellTyped::Array(_) => {}
            XCellTyped::Custom(v) => v.default.to_string(),
        }
    }
    fn make_cs_binary_writer(&self, field: &str) -> Vec<String> {
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
    fn make_cs_binary_reader(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "r.ReadBoolean()".to_string(),
            XCellTyped::Integer(v) => format!("r.{}()", v.as_csharp_reader()),
            XCellTyped::Decimal(v) => format!("r.{}()", v.as_csharp_reader()),
            XCellTyped::String(_) => "r.ReadString()".to_string(),
            XCellTyped::Time(_) => "new DateTime(r.ReadInt64(), DateTimeKind.Utc)".to_string(),
            XCellTyped::Color(_) => "new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())".to_string(),
            XCellTyped::Enumerate(v) => v.default.to_string(),
            XCellTyped::Custom(v) => v.default.to_string(),
        }
    }
}

impl IntegerKind {}

impl TimeDescription {
    fn make_cs_datetime(&self) -> String {
        let y = self.default.year();
        let m = self.default.month();
        let d = self.default.day();
        let h = self.default.hour();
        let min = self.default.minute();
        let s = self.default.second();
        format!("new DateTime({y}, {m}, {d}, {h}, {min}, {s})")
    }
}

impl ColorDescription {
    fn make_cs_color32(&self) -> String {
        let [r, g, b, a] = self.default.to_rgba8();
        format!("new Color32({r}, {g}, {b}, {a})")
    }
}
