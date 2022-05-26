use std::{fs::File, path::Path};

use crate::{
    typing::{XCellTyped, XCellValue},
    XCellHeader,
};

use super::*;

impl UnityCodegen {
    pub fn write_xml(&self, table: &XCellTable, f: &mut impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        for header in &table.headers {
            header.write_csharp(&mut file)?;
        }
        Ok(())
    }
    pub fn write_interface(&self, f: &mut impl Write) -> std::io::Result<usize> {
        let mut slots = HashMap::new();
        slots.insert("NAMESPACE", self.namespace.join("."));
        let render = build_template(include_str!("DefineInterface.cs")).render_nofail(&slots);
        f.write(render.as_bytes())
    }
}

impl XCellHeader {
    pub fn write_csharp(&self, f: &mut impl Write) -> std::io::Result<()> {
        self.typing.write_csharp(f, &self.field_name)
    }
}

impl XCellTyped {
    pub fn write_csharp(&self, f: &mut impl Write, field: &str) -> std::io::Result<()> {
        match self {
            XCellTyped::Boolean(v) => {
                writeln!(f, "public bool {} = {};", field, v.default)
            }
            XCellTyped::Integer8(_) => {
                todo!()
            }
            XCellTyped::Integer16(_) => {
                todo!()
            }
            XCellTyped::Integer32(v) => {
                writeln!(f, "public int {} = {};", field, v.default)
            }
            XCellTyped::Integer64(_) => {
                todo!()
            }
            XCellTyped::Unsigned8(_) => {
                todo!()
            }
            XCellTyped::Unsigned16(_) => {
                todo!()
            }
            XCellTyped::Unsigned32(v) => {
                writeln!(f, "public uint {} = {};", field, v.default)
            }
            XCellTyped::Unsigned64(v) => {
                writeln!(f, "public ulong {} = {};", field, v.default)
            }
            XCellTyped::Float32(_) => {
                todo!()
            }
            XCellTyped::Float64(_) => {
                todo!()
            }
            XCellTyped::Decimal128(_) => {
                todo!()
            }
            XCellTyped::String(v) => {
                writeln!(f, "public string {} = {};", field, v.default)
            }
            XCellTyped::LanguageID(v) => {
                writeln!(f, "public string {} = {};", field, v.default)
            }
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                todo!()
            }
            XCellTyped::Custom(v) => {
                writeln!(f, "public {} {} = {};", v.typing, field, v.default)
            }
        }
    }
}

impl XCellValue {}
