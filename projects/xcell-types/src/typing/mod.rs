use std::{
    fmt::{Debug, Display, Formatter},
};

use serde::{Deserialize, Serialize};

use xcell_errors::for_3rd::DataType;

pub use crate::{
    array::{ArrayDescription, ArrayKind},
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
    value::{
        boolean::BooleanDescription, color::ColorDescription, custom::CustomDescription, enumerate::EnumerateDescription,
        string::StringDescription,
    },
    vector::VectorDescription,
};
pub use crate::value::time::TimeDescription;

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

impl XCellTyped {
    pub fn parse(field: &str, ty: &DataType) -> Self {
        todo!()
    }
}
