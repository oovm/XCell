use crate::{XError, XErrorKind::SyntaxError};
use globset::Error;
pub use globset::{Glob, GlobSetBuilder, GlobSet};

impl From<Error> for XError {
    fn from(e: Error) -> Self {
        XError { kind: Box::new(SyntaxError { message: e.to_string() }), path: None, position: None, source: Some(Box::new(e)) }
    }
}
