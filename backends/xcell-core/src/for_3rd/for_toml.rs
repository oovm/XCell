use crate::{XError, XErrorKind};
use oak_core::OakError;

impl From<OakError> for XError {
    fn from(e: OakError) -> Self {
        Self { kind: Box::new(XErrorKind::from(&e)), path: None, position: None, source: Some(Box::new(e)) }
    }
}

impl From<&OakError> for XErrorKind {
    fn from(e: &OakError) -> Self {
        XErrorKind::SyntaxError { message: e.to_string() }
    }
}
