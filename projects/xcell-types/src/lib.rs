pub use self::{
    array::{ArrayDescription, ArrayKind},
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
    typing::XCellTyped,
    value::{color::ColorDescription, time::TimeDescription, XCellValue},
    vector::VectorDescription,
};

pub(crate) mod errors;

mod array;
mod decimal;
mod integer;
mod typing;
mod value;
mod vector;
