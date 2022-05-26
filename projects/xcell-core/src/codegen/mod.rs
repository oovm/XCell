use std::{collections::HashMap, io::Write};

use new_string_template::template::Template;
use regex::Regex;

use crate::{XCellTable, XError};

#[allow(unused_variables)]
mod csv;
#[allow(unused_variables)]
mod unity;

pub struct UnityCodegen {
    pub namespace: Vec<String>,
    pub support_binary: bool,
}

impl Default for UnityCodegen {
    fn default() -> Self {
        Self { namespace: vec!["DataTable".to_string()], support_binary: true }
    }
}

pub struct CsvCodegen {}

pub struct BinaryCodegen {}

pub fn build_template(template: &str) -> Template {
    let db_line = Regex::new(r"(?mi)__\s+([^_]+)\s+__").unwrap();
    Template::new(template).with_regex(&db_line)
}
