use tera::{Error, ErrorKind};

use crate::{XError, XErrorKind};

impl From<Error> for XError {
    fn from(e: Error) -> Self {
        let kind = box match e.kind {
            ErrorKind::Io(e) => XErrorKind::IOError(std::io::Error::from(e)),
            _ => XErrorKind::SyntaxError(e.to_string()),
        };
        Self { kind, path: None, position: None }
    }
}
