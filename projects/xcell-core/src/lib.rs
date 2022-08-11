#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(once_cell)]
#![feature(path_file_prefix)]
#![feature(file_create_new)]

pub use xcell_errors::{Failure, Success, Validation, XError, XErrorKind, XResult};
pub use xcell_types::*;

pub use self::{
    config::WorkspaceManager,
    x_table::{data::*, header::XCellHeader, table::XCellTable},
};

pub mod codegen;
pub mod config;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;
