use std::{fs::File, io::Write, path::Path};

use byteorder::{LittleEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use toml::Value;
use convert_case::{Case, Casing};
use crate::{
    typing::{XCellTyped, XCellValue},
    ColorDescription, XCellHeader, XCellHeaders, XCellTable, XError, XResult,
};

pub use self::unity::UnityCodegen;

mod binary;
mod readable;
mod unity;

pub struct CsvCodegen {}

pub struct BinaryCodegen {}

// pub fn build_template(template: &str) -> Template {
//     let db_line = Regex::new(r"(?mi)__\s+([^_]+)\s+__").unwrap();
//     Template::new(template).with_regex(&db_line)
// }

fn tera_render(template: &str, slots: &Context, output: &Path) -> XResult<String> {
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
pub fn split_file_name(s: &str) -> String {
    let mut all = vec![];
    for name in s.split(|c| c == '/' || c == '\\') {
        if !name.trim().is_empty() {
            all.push(name)
        }
    }
    all.join("/")
}
pub fn split_namespace(s: &str) -> Vec<&str> {
    let mut all = vec![];
    for s in s.split("::") {
        for name in s.split('.') {
            if !name.trim().is_empty() {
                all.push(name)
            }
        }
    }
    all
}
