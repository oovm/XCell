use std::path::PathBuf;

use crate::XCellType;

mod for_calamine;
mod for_std;
mod for_toml;

#[derive(Debug)]
pub struct XError {
    pub kind: Box<XErrorKind>,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum XErrorKind {
    IOError(std::io::Error),
    TableError(String),
    TypeMismatch { x: u32, y: u32, except: XCellType, current: XCellType },
    UnknownError,
}

pub type XResult<T = ()> = Result<T, XError>;

impl XError {
    pub fn table_error<S: Into<String>>(msg: S, path: &PathBuf) -> Self {
        Self { kind: box XErrorKind::TableError(msg.into()), path: Some(path.clone()) }
    }
    pub fn type_mismatch(x: u32, y: u32, except: XCellType, current: XCellType, path: &PathBuf) -> XError {
        Self { kind: box XErrorKind::TypeMismatch { x, y, except, current }, path: Some(path.clone()) }
    }
}
