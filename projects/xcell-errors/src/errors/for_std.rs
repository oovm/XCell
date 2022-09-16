use std::io::Error as IOError;

use crate::{XError, XErrorKind};

impl Default for XErrorKind {
    fn default() -> Self {
        XErrorKind::UnknownError
    }
}

impl From<()> for XError {
    #[track_caller]
    fn from(_: ()) -> Self {
        let caller_location = std::panic::Location::caller();
        XError::new(XErrorKind::RuntimeError {
            message: format!("空指针调用: {}", caller_location),
        })
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
