#![feature(box_syntax)]

pub mod codegen;
mod errors;
mod x_table;
pub use self::x_table::{typing::XCellType, XCellHeader, XCellTable};
pub use errors::{XError, XErrorKind, XResult};
