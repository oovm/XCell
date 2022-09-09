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

use crate::{utils::push_delimiter, XCellTyped, XCellValueKind};

mod der;
mod parse_cell;

#[derive(Debug, Clone, Serialize)]
pub struct VectorDescription {
    delimiter: BTreeSet<char>,
    suffix: BTreeSet<String>,
    typing: XCellTyped,
    pub default: Vec<XCellValueKind>,
}

impl VectorDescription {
    pub fn add_delimiter(&mut self, set: &str) {
        push_delimiter(&mut self.delimiter, set)
    }
    pub fn add_suffix<S>(&mut self, suffix: S)
    where
        S: Into<String>,
    {
        self.suffix.insert(suffix.into());
    }
    pub fn matches_rest<'i>(&self, s: &'i str) -> Option<&'i str> {
        for suffix in &self.suffix {
            if s.to_ascii_lowercase().ends_with(&suffix.to_ascii_lowercase()) {
                return Some(&s[0..s.len() - suffix.len()]);
            }
        }
        None
    }
    pub fn get_type(&self) -> &XCellTyped {
        &self.typing
    }
    pub fn set_type<T>(&mut self, typing: T)
    where
        T: Into<XCellTyped>,
    {
        self.typing = typing.into()
    }
    pub fn with_type<T>(mut self, typing: T) -> Self
    where
        T: Into<XCellTyped>,
    {
        self.typing = typing.into();
        self
    }
}
