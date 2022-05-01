use crate::{XError, XErrorKind};

impl From<std::io::Error> for XError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: Box::new(XErrorKind::IOError(e)), path: None }
    }
}
