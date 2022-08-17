use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use serde_types::OneOrMany;
use xcell_errors::{
    for_3rd::{read_map_next_extra, read_map_next_value, DataType},
    XResult,
};

use crate::{utils::push_delimiter, XCellTyped, XCellValue};

mod der;
mod parse_cell;

#[derive(Debug, Clone, Serialize)]
pub struct VectorDescription {
    delimiter: BTreeSet<char>,
    suffix: BTreeSet<String>,
    pub typing: XCellTyped,
    pub default: Vec<XCellValue>,
}

impl VectorDescription {
    pub fn add_delimiter(&mut self, set: &str) {
        push_delimiter(&mut self.delimiter, set)
    }
    pub fn add_suffix<S>(&mut self, suffix: &str)
    where
        S: Into<String>,
    {
        self.suffix.insert(suffix.into());
    }
}
