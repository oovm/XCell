use walkdir::Error;
pub use walkdir::{DirEntry, WalkDir};

use crate::{XError, XErrorKind};

impl From<Error> for XError {
    fn from(error: Error) -> Self {
        let path = error.path().map(|s| s.to_path_buf());
        Self { kind: Box::new(XErrorKind::IOError(error.to_string())), path, position: None, source: Some(Box::new(error)) }
    }
}
