use std::{
    any::type_name,
    collections::BTreeSet,
    convert::Infallible,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use serde_types::OneOrMany;
use xcell_errors::{
    for_3rd::{read_map_next_extra, read_map_next_value, DataType},
    XResult,
};

pub use crate::{
    array::{ArrayDescription, ArrayKind},
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
    string::StringDescription,
    value::{color::ColorDescription, enumerate::EnumerateDescription, time::TimeDescription},
    vector::VectorDescription,
};
use crate::{BooleanDescription, XCellValue};

mod der;
mod display;
mod parser;

#[derive(Clone, Serialize, Deserialize)]
pub enum XCellTyped {
    Boolean(Box<BooleanDescription>),
    Integer(Box<IntegerDescription>),
    Decimal(Box<DecimalDescription>),
    String(Box<StringDescription>),
    Time(Box<TimeDescription>),
    Color(Box<ColorDescription>),
    Enumerate(Box<EnumerateDescription>),
    Array(Box<ArrayDescription>),
    Vector(Box<VectorDescription>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum XTableKind {
    SortedMap,
    Enumerate,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ExtraTypes {
    pub string: BTreeSet<String>,
    pub vector: BTreeSet<String>,
}

impl ExtraTypes {
    pub fn is_string<'a>(&self, typing: &'a str) -> Option<&'a str> {
        for v in &self.string {
            if typing.eq_ignore_ascii_case(v) {
                return Some(typing);
            }
        }
        None
    }
    pub fn is_vector<'a>(&self, typing: &'a str) -> Option<&'a str> {
        for v in &self.vector {
            if typing.ends_with(v) {
                return Some(typing.trim_end_matches(v));
            }
        }
        None
    }
}

impl XCellTyped {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {
        match self {
            XCellTyped::Boolean(typing) => typing.parse_cell(cell),
            XCellTyped::Integer(typing) => typing.parse_cell(cell),
            XCellTyped::Decimal(typing) => typing.parse_cell(cell),
            XCellTyped::String(typing) => typing.parse_cell(cell),
            XCellTyped::Time(typing) => typing.parse_cell(cell),
            XCellTyped::Color(typing) => typing.parse_cell(cell),
            XCellTyped::Enumerate(typing) => typing.parse_cell(cell).map(XCellValue::String),
            XCellTyped::Array(typing) => typing.parse_cell(cell),
            XCellTyped::Vector(_) => {
                todo!()
            }
        }
    }
}
