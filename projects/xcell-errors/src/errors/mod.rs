use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::PathBuf,
};

mod for_std;

#[derive(Debug, Default)]
pub struct XError {
    pub kind: Box<XErrorKind>,
    pub path: Option<PathBuf>,
    pub position: Option<(usize, usize)>,
    pub source: Option<Box<dyn Error>>,
}

#[derive(Debug)]
pub enum XErrorKind {
    IOError(String),
    SyntaxError { message: String },
    TableError(String),
    TypeMismatch { except: String, current: String },
    UnknownError,
}

impl Display for XError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for XError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(s) => Some(s.as_ref()),
            None => None,
        }
    }
}

impl XError {
    pub fn new(kind: XErrorKind) -> Self {
        Self { kind: Box::new(kind), path: None, position: None, source: None }
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }
    pub fn with_xy(mut self, x: usize, y: usize) -> Self {
        self.position = Some((x, y));
        self
    }
    // pub fn table_error<S>(msg: S) -> Self
    // where
    //     S: Into<String>,
    // {
    //     Self { kind: box XErrorKind::TableError(msg.into()), path: None, position: None }
    // }
    // pub fn type_mismatch(except: XCellTyped, current: DataType, x: usize, y: usize, path: PathBuf) -> XError {
    //     Self { kind: box XErrorKind::TypeMismatch { except, current }, path: Some(path), position: Some((x, y)) }
    // }
}
