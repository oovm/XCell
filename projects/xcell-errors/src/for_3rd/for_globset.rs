use diagnostic::{Failure, Success};
pub use globset::GlobSet;
use globset::{Error, Glob, GlobSetBuilder};

use crate::{Validation, XError, XErrorKind::SyntaxError};

impl From<Error> for XError {
    fn from(e: Error) -> Self {
        XError { kind: Box::new(SyntaxError { message: e.to_string() }), path: None, position: None, source: Some(Box::new(e)) }
    }
}

pub fn build_glob_set(patterns: &str) -> Validation<GlobSet> {
    let mut diagnostics = vec![];
    let mut builder = GlobSetBuilder::new();
    for line in patterns.lines() {
        match Glob::new(line) {
            Ok(o) => {
                builder.add(o);
            }
            Err(e) => diagnostics.push(XError::from(e)),
        }
    }
    match builder.build() {
        Ok(o) => Success { value: o, diagnostics },
        Err(e) => Failure { fatal: XError::from(e), diagnostics },
    }
}
