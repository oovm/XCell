use crate::{XError, XErrorKind};
use toml::de::Error;

impl From<Error> for XError {
    fn from(e: Error) -> Self {
        Self { kind: Box::new(XErrorKind::from(&e)), path: None, position: None, source: Some(Box::new(e)) }
    }
}

impl From<&Error> for XErrorKind {
    fn from(e: &Error) -> Self {
        XErrorKind::SyntaxError { message: e.to_string() }
    }
}
