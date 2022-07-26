#![allow(clippy::get_first)]

pub use stream_io::{ByteOrder, StreamReader, StreamWriter};

pub use self::{
    array::{ArrayDescription, ArrayKind},
    boolean::BooleanDescription,
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
    string::StringDescription,
    typing::*,
    value::{color::ColorDescription, time::TimeDescription, XCellValue},
    vector::VectorDescription,
};

pub(crate) mod utils;

mod array;
mod boolean;
mod codegen;
pub mod custom;
mod decimal;
mod integer;
pub mod string;
mod typing;
mod value;
mod vector;
