use tera::{Error, ErrorKind};

use crate::{XError, XErrorKind};

impl From<Error> for XError {
    fn from(error: Error) -> Self {
        let mut out = XError::default();
        match error.kind {
            ErrorKind::Io(e) => {
                let error = std::io::Error::from(e);
                out.kind = Box::new(XErrorKind::from(&error));
            }
            _ => {
                out.kind = Box::new(XErrorKind::SyntaxError { message: error.to_string() });
                out.source = Some(Box::new(error))
            }
        }
        out
    }
}
