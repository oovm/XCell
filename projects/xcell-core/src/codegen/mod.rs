use std::{fs::File, io::Write, path::Path};

use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use xcell_errors::XResult;
use xcell_types::XCellTyped;

pub use crate::config::UnityCodegen;
use crate::{XCellHeader, XCellHeaders, XCellTable};

#[allow(unused)]
mod binary;
#[allow(unused)]
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

pub fn write_newline(f: &mut impl Write) -> std::io::Result<usize> {
    f.write(b"\n")
}
