#![feature(box_syntax)]

use crate::x_table::meta_info::{BooleanMetaInfo, TypeMetaInfo};

pub use self::{
    errors::{XError, XErrorKind, XResult},
    x_table::{XCellHeader, XCellMetaInfo, XCellTable, XCellType},
};

pub mod codegen;
mod errors;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;
