#![feature(box_syntax)]

pub use self::{
    errors::{XError, XErrorKind, XResult},
    x_table::{XCellHeader, XCellTable, XCellType},
};

pub mod codegen;
mod errors;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;
