use toml::de::Error;

use crate::{XError, XErrorKind};

impl From<Error> for XError {
    fn from(e: Error) -> Self {
        XError { kind: Box::new(XErrorKind::TableError(e.to_string())), path: None, position: None }
    }
}
