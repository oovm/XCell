#![feature(box_syntax)]
#![feature(try_blocks)]
#![feature(type_alias_impl_trait)]
#![feature(once_cell)]
#![feature(path_file_prefix)]

pub use xcell_errors::{Failure, Success, Validation, XError, XErrorKind, XResult};

pub use self::{
    config::{BooleanMetaInfo, TableConfig, TypeMetaInfo, WorkspaceManager},
    x_table::{XCellHeader, XCellHeaders, XCellTable},
};

pub mod codegen;
pub mod config;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;
