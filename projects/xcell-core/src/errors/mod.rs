use std::path::PathBuf;

use crate::typing::XCellTyped;

mod for_calamine;
mod for_std;
mod for_toml;

#[derive(Debug)]
pub struct XError {
    pub kind: Box<XErrorKind>,
    pub path: Option<PathBuf>,
    pub position: Option<(u32, u32)>,
}

#[derive(Debug)]
pub enum XErrorKind {
    IOError(std::io::Error),
    TableError(String),
    TypeMismatch { except: XCellTyped, current: XCellTyped },
    UnknownError,
}

pub type XResult<T = ()> = Result<T, XError>;

impl XError {
    pub fn with_path(self, path: PathBuf) -> Self {
        Self { kind: self.kind, path: Some(path), position: self.position }
    }
    pub fn with_xy(self, x: u32, y: u32) -> Self {
        Self { kind: self.kind, path: self.path, position: Some((x, y)) }
    }
    pub fn table_error<S: Into<String>>(msg: S) -> Self {
        Self { kind: box XErrorKind::TableError(msg.into()), path: None, position: None }
    }
    pub fn type_mismatch(except: XCellTyped, current: XCellTyped, x: u32, y: u32, path: PathBuf) -> XError {
        Self { kind: box XErrorKind::TypeMismatch { except, current }, path: Some(path), position: Some((x, y)) }
    }
}
