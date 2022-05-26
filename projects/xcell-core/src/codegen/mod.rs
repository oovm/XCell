use std::{collections::HashMap, fs::File, io::Write, path::Path};

use byteorder::WriteBytesExt;
use new_string_template::template::Template;
use regex::Regex;

use crate::{
    typing::{XCellTyped, XCellValue},
    XCellHeader, XCellTable, XError,
};

#[allow(unused_variables)]
mod csv;
#[allow(unused_variables)]
mod unity;

pub struct UnityCodegen {
    pub namespace: Vec<String>,
    pub namespace_legacy: bool,
    pub table_suffix: String,
    pub element_suffix: String,
    pub support_binary: bool,
}

impl Default for UnityCodegen {
    fn default() -> Self {
        Self {
            namespace: vec!["DataTable".to_string()],
            support_binary: true,
            namespace_legacy: true,
            table_suffix: "Table".to_string(),
            element_suffix: "Element".to_string(),
        }
    }
}

pub struct CsvCodegen {}

pub struct BinaryCodegen {}

pub fn build_template(template: &str) -> Template {
    let db_line = Regex::new(r"(?mi)__\s+([^_]+)\s+__").unwrap();
    Template::new(template).with_regex(&db_line)
}

pub fn write_newline(f: &mut impl Write) -> std::io::Result<()> {
    f.write_u8(b'\n')
}
