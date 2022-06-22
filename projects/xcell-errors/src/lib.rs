pub use errors::{XError, XErrorKind};

mod errors;
pub mod for_3rd;

pub type XResult<T = ()> = Result<T, XError>;

pub type Validation<T> = diagnostic::Validation<T, XError>;
