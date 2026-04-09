use crate::{XError, XErrorKind};
use oak_toml::parser::error::ParseError;

impl From<ParseError> for XError {
    fn from(e: ParseError) -> Self {
        Self { kind: Box::new(XErrorKind::from(&e)), path: None, position: None, source: Some(Box::new(e)) }
    }
}

impl From<&ParseError> for XErrorKind {
    fn from(e: &ParseError) -> Self {
        XErrorKind::SyntaxError { message: e.to_string() }
    }
}
