use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use xcell_errors::{for_3rd::DataType, XResult};

pub use crate::{
    array::{ArrayDescription, ArrayKind},
    decimal::{DecimalDescription, DecimalKind},
    enumerate::EnumerateDescription,
    integer::{IntegerDescription, IntegerKind},
    string::StringDescription,
    value::{color::ColorDescription, time::TimeDescription},
    vector::VectorDescription,
};
use crate::{BooleanDescription, XCellValueKind};

mod display;
mod parser;
use serde::de::{MapAccess, Visitor};
use std::any::type_name;
use xcell_errors::for_3rd::{read_map_next_extra, read_map_next_key_lowercase, read_map_next_value};

mod der;
mod ser;

#[derive(Debug, Clone, Default, Serialize)]
pub struct TypeMetaInfo {
    pub boolean: BooleanDescription,
    pub string: StringDescription,
    pub vector: VectorDescription,
}

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

impl Default for XCellTyped {
    fn default() -> Self {
        Self::Boolean(Box::default())
    }
}

impl XCellTyped {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValueKind> {
        match self {
            XCellTyped::Boolean(typing) => typing.parse_cell(cell),
            XCellTyped::Integer(typing) => typing.parse_cell(cell),
            XCellTyped::Decimal(typing) => typing.parse_cell(cell),
            XCellTyped::String(typing) => typing.parse_cell(cell),
            XCellTyped::Time(typing) => typing.parse_cell(cell),
            XCellTyped::Color(typing) => typing.parse_cell(cell),
            XCellTyped::Enumerate(typing) => typing.parse_cell(cell),
            XCellTyped::Array(typing) => typing.parse_cell(cell),
            XCellTyped::Vector(typing) => typing.parse_cell(cell),
        }
    }
}
