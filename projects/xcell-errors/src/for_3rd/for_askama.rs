use crate::XError;
use askama::Error;

impl From<Error> for XError {
    fn from(e: Error) -> Self {
        XError::runtime_error(e.to_string())
    }
}
