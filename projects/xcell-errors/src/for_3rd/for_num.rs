use num::bigint::ParseBigIntError;
pub use num::{BigInt, FromPrimitive, ToPrimitive, Zero};

use crate::{XError, XErrorKind};

impl From<ParseBigIntError> for XError {
    fn from(value: ParseBigIntError) -> Self {
        Self {
            kind: Box::new(XErrorKind::SyntaxError { message: value.to_string() }),
            path: None,
            position: None,
            source: None,
        }
    }
}
