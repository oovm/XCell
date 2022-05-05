use calamine::Error;

use crate::{errors::XError, XErrorKind};

impl From<Error> for XError {
    fn from(calamine: Error) -> Self {
        let kind = match calamine {
            Error::Io(e) => XErrorKind::IOError(e),
            Error::Ods(_) => {
                todo!()
            }
            Error::Xls(_) => {
                todo!()
            }
            Error::Xlsb(_) => {
                todo!()
            }
            Error::Xlsx(_) => {
                todo!()
            }
            Error::Vba(_) => {
                todo!()
            }
            Error::De(_) => {
                todo!()
            }
            Error::Msg(e) => XErrorKind::TableError(e.to_string()),
        };
        XError { kind: box kind, path: None }
    }
}
