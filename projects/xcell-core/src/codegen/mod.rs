use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::Serialize;
use tera::{Context, Tera};

use xcell_errors::XResult;
use xcell_types::codegen::{CSharpReader, CSharpWriter};

use crate::{
    config::{TableMerged, UnityCodegen},
    XCellHeader, XCellTable, XData,
};

pub mod binary;
pub mod readable;
pub mod unity;
pub mod xml;

pub struct CsvCodegen {}

pub struct BinaryCodegen {}

// pub fn build_template(template: &str) -> Template {
//     let db_line = Regex::new(r"(?mi)__\s+([^_]+)\s+__").unwrap();
//     Template::new(template).with_regex(&db_line)
// }

fn tera_render(template: &str, slots: &Context, output: &Path, name: &str) -> XResult<String> {
    // log::trace!("tera_render {}", output.display());
    let mut file = File::create(output)?;
    let mut tera = Tera::default();
    // tera.autoescape_on(vec![]);
    tera.add_raw_template(name, template).unwrap();
    let result = tera.render(name, slots).unwrap();
    file.write_all(result.as_bytes())?;
    Ok(result)
}

pub fn write_newline(f: &mut impl Write) -> std::io::Result<usize> {
    f.write(b"\n")
}
