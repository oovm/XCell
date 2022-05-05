#![feature(box_syntax)]

use self::x_table::global_config::ProjectConfig;
pub use self::{
    errors::{XError, XErrorKind, XResult},
    x_table::{
        table_config::{BooleanMetaInfo, TableConfig, TypeMetaInfo},
        XCellHeader, XCellTable,
    },
};

pub mod codegen;
mod errors;
pub mod typing;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;
