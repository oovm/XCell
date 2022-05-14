#![feature(box_syntax)]
#![feature(try_blocks)]

pub use diagnostic::Validation::{Failure, Success};

pub use self::{
    errors::{Validation, XError, XErrorKind, XResult},
    typing::boolean::BooleanDescription,
    x_table::{
        table_config::{BooleanMetaInfo, ProjectConfig, TableConfig, TypeMetaInfo},
        XCellHeader, XCellTable,
    },
};

pub mod codegen;
mod errors;
pub mod typing;
pub mod utils;
mod x_table;

pub type CalamineTable = calamine::Range<calamine::DataType>;
