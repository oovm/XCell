pub use calamine::DataType;
use calamine::Error;

use crate::{errors::XError, XErrorKind};

impl From<Error> for XError {
    fn from(error: Error) -> Self {
        let mut out = XError::default();
        match error {
            Error::Io(e) => {
                out.kind = Box::new(XErrorKind::from(&e));
                out.source = Some(Box::new(e))
            }
            Error::Ods(e) => out.source = Some(Box::new(e)),
            Error::Xls(e) => out.source = Some(Box::new(e)),
            Error::Xlsb(e) => out.source = Some(Box::new(e)),
            Error::Xlsx(e) => out.source = Some(Box::new(e)),
            Error::Vba(e) => out.source = Some(Box::new(e)),
            Error::De(e) => out.source = Some(Box::new(e)),
            Error::Msg(e) => {
                out.kind = Box::new(XErrorKind::TableError(e.to_string()));
                out.source = Some(Box::new(error))
            }
        };
        out
    }
}
