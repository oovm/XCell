#![allow(clippy::get_first)]

pub use stream_io::{ByteOrder, StreamReader, StreamWriter};

pub use self::{
    array::{ArrayDescription, ArrayKind},
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
    typing::*,
    value::{color::ColorDescription, time::TimeDescription, XCellValue},
    vector::VectorDescription,
};

pub(crate) mod errors;

mod array;
mod codegen;
pub mod custom;
mod decimal;
mod integer;
mod typing;
mod value;
mod vector;
