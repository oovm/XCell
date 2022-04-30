#![feature(box_syntax)]

pub use self::{
    errors::{XError, XErrorKind, XResult},
    x_table::{XCellHeader, XCellTable, XCellType},
};

pub mod codegen;
mod errors;
mod x_table;
