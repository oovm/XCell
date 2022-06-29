use std::{
    convert::Infallible,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use xcell_errors::{for_3rd::DataType, XResult};

use crate::XCellValue;
pub use crate::{
    array::{ArrayDescription, ArrayKind},
    custom::CustomDescription,
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
    value::{
        boolean::BooleanDescription, color::ColorDescription, enumerate::EnumerateDescription, string::StringDescription,
        time::TimeDescription,
    },
    vector::VectorDescription,
};

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
    Custom(Box<CustomDescription>),
}

mod display;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XTableKind {
    SortedMap,
    Enumerate,
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
            XCellTyped::Custom(typing) => typing.parse_cell(cell).map(XCellValue::String),
            XCellTyped::Array(_) => {
                todo!()
            }
            XCellTyped::Vector(_) => {
                todo!()
            }
        }
    }
}
