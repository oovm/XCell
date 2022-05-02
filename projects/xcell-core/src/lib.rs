#![feature(box_syntax)]

pub use self::{
    errors::{XError, XErrorKind, XResult},
    x_table::{
        meta_info::{BooleanMetaInfo, TypeMetaInfo},
        XCellHeader, XCellMetaInfo, XCellTable, XCellType,
    },
};

pub mod codegen;
mod errors;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;
