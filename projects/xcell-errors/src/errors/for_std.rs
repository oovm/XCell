use std::io::Error as IOError;

use crate::{XError, XErrorKind};

impl Default for XErrorKind {
    fn default() -> Self {
        XErrorKind::UnknownError
    }
}

impl From<&IOError> for XErrorKind {
    fn from(e: &IOError) -> Self {
        XErrorKind::IOError(e.to_string())
    }
}

impl From<IOError> for XError {
    fn from(e: IOError) -> Self {
        Self { kind: Box::new(XErrorKind::from(&e)), path: None, position: None, source: Some(Box::new(e)) }
    }
}
