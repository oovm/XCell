use calamine::DataType;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::PathBuf,
};

use crate::typing::XCellTyped;

mod for_calamine;
mod for_std;
mod for_toml;
mod for_tera;

#[derive(Debug)]
pub struct XError {
    pub kind: Box<XErrorKind>,
    pub path: Option<PathBuf>,
    pub position: Option<(usize, usize)>,
}

#[derive(Debug)]
pub enum XErrorKind {
    IOError(std::io::Error),
    SyntaxError(String),
    TableError(String),
    TypeMismatch { except: XCellTyped, current: DataType },
    UnknownError,
}

pub type XResult<T = ()> = Result<T, XError>;

pub type Validation<T> = diagnostic::Validation<T, XError>;

impl XError {
    pub fn with_path(self, path: PathBuf) -> Self {
        Self { kind: self.kind, path: Some(path), position: self.position }
    }
    pub fn with_xy(self, x: usize, y: usize) -> Self {
        Self { kind: self.kind, path: self.path, position: Some((x, y)) }
    }
    pub fn table_error<S: Into<String>>(msg: S) -> Self {
        Self { kind: box XErrorKind::TableError(msg.into()), path: None, position: None }
    }
    pub fn type_mismatch(except: XCellTyped, current: DataType, x: usize, y: usize, path: PathBuf) -> XError {
        Self { kind: box XErrorKind::TypeMismatch { except, current }, path: Some(path), position: Some((x, y)) }
    }
}

impl Display for XError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for XError {}
