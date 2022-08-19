use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use convert_case::{Case, Casing};

use serde::Serialize;
use serde_json::Value;
use tera::{Context, Tera};

use xcell_errors::XResult;
use xcell_types::{
    codegen::{CSharpReader, CSharpWriter},
    ByteOrder, StreamWriter,
};

use crate::{UnityCodegen, XCellHeader, XCellTable, XData};

pub mod binary;
pub mod readable;
pub mod unity;
pub mod xml;

pub struct CsvCodegen {}

fn tera_render(template: &str, slots: &Context, output: &Path, name: &str) -> XResult<String> {
    // log::trace!("tera_render {}", output.display());
    let mut file = File::create(output)?;
    let mut tera = Tera::default();
    // tera.autoescape_on(vec![]);
    tera.add_raw_template(name, template).unwrap();
    tera.register_filter("camel_case", camel_case);
    let result = tera.render(name, slots).unwrap();
    file.write_all(result.as_bytes())?;
    Ok(result)
}

pub fn write_newline(f: &mut impl Write) -> std::io::Result<usize> {
    f.write(b"\n")
}

fn camel_case(input: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    match input {
        Value::String(v) => Ok(Value::String(v.to_case(Case::Camel))),
        _ => panic!(),
    }
}
