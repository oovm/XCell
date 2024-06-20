use crate::{XError, XErrorKind};
pub use csscolorparser::Color;
use csscolorparser::ParseColorError;

impl From<ParseColorError> for XError {
    fn from(e: ParseColorError) -> Self {
        Self { kind: Box::new(XErrorKind::from(&e)), path: None, position: None, source: Some(Box::new(e)) }
    }
}

impl From<&ParseColorError> for XErrorKind {
    fn from(e: &ParseColorError) -> Self {
        XErrorKind::SyntaxError { message: e.to_string() }
    }
}
