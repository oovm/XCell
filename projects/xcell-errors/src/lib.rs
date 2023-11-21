pub use errors::{XError, XErrorKind};

mod errors;
pub mod for_3rd;

pub use validatus::Validation::{Failure, Success};

pub type XResult<T = ()> = Result<T, XError>;

pub type Validation<T> = validatus::Validation<T, XError>;
