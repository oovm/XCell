use crate::XClassItem;
use convert_case::{Case, Casing};
use itertools::Itertools;
use serde::Serialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};
use tera::{Context, Tera};
use url::Url;
use xcell_errors::XResult;
use xcell_types::{
    codegen::{CSharpReader, CSharpWriter},
    ByteOrder, StreamWriter, XCellValue,
};

use crate::{BinaryWriter, DataContractWriter, MergedTable, UnityCodegen, XCellHeader, XDataItem, XExportData, XTable};

pub mod binary;
pub mod readable;
pub mod unity;
pub mod xml;

pub struct CsvCodegen {}

fn tera_render(template: &str, slots: &Context, output: &Path, name: &str) -> XResult<String> {
    let mut file = File::create(output)?;
    let mut tera = Tera::default();
    tera.add_raw_template(name, &template.replace("\r\n", "\n")).unwrap();
    tera.register_filter("public_name", public_name);
    tera.register_filter("private_name", private_name);
    let result = tera.render(name, slots).unwrap();
    file.write_all(result.as_bytes())?;
    Ok(result)
}

fn public_name(input: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let name = input.as_str().ok_or("Not String")?;
    Ok(Value::String(name.to_case(Case::Camel)))
}

fn private_name(input: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let name = input.as_str().ok_or("Not String")?;
    Ok(Value::String(format!("_{}", name.to_case(Case::Snake))))
}
