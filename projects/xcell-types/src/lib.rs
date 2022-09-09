#![allow(clippy::get_first)]

pub use stream_io::{ByteOrder, StreamReader, StreamWriter};

pub use self::{
    array::{ArrayDescription, ArrayKind},
    boolean::BooleanDescription,
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
    string::StringDescription,
    typing::*,
    value::{color::ColorDescription, time::TimeDescription, XCellValueKind},
    vector::VectorDescription,
};
pub use xcell_errors::for_3rd::DateTime;

pub(crate) mod utils;

mod array;
mod boolean;
pub mod codegen;
mod custom;
mod decimal;
pub mod enumerate;
mod integer;
mod string;
mod typing;
mod value;
mod vector;
