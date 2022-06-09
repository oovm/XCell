use std::{fs::File, io::Write, path::Path};

use byteorder::{LittleEndian, WriteBytesExt};
use serde::Serialize;
use tera::{Context, Tera};

use crate::{
    typing::{XCellTyped, XCellValue},
    ColorDescription, XCellHeader, XCellHeaders, XCellTable, XError, XResult,
};

mod binary;
mod readable;
mod unity;

pub struct UnityCodegen {
    pub namespace: Vec<String>,
    pub table_suffix: String,
    pub element_suffix: String,
    pub support_binary: bool,
    pub support_clone: bool,
}

impl Default for UnityCodegen {
    fn default() -> Self {
        Self {
            namespace: vec!["DataTable".to_string(), "Generated".to_string()],
            support_binary: true,
            table_suffix: "Table".to_string(),
            element_suffix: "Element".to_string(),
            support_clone: true,
        }
    }
}

pub struct CsvCodegen {}

pub struct BinaryCodegen {}

// pub fn build_template(template: &str) -> Template {
//     let db_line = Regex::new(r"(?mi)__\s+([^_]+)\s+__").unwrap();
//     Template::new(template).with_regex(&db_line)
// }

fn tera_render(template: &str, slots: &Context, output: &Path)  -> XResult<String> {
    let mut file = File::create(output)?;
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template("T", template)?;
    let result = tera.render("T", slots)?;
    file.write_all(result.as_bytes())?;
    Ok(result)
}

pub fn write_newline(f: &mut impl Write) -> std::io::Result<()> {
    f.write_u8(b'\n')
}
