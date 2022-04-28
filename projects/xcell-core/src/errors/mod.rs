use lsp_types::Url;

mod for_calamine;

#[derive(Debug)]
pub struct XError {
    pub kind: Box<XErrorKind>,
    pub path: Option<Url>,
}

#[derive(Debug)]
pub enum XErrorKind {
    IOError(std::io::Error),
    TableError(String),
    UnknownError,
}

pub type XResult<T = ()> = Result<T, XError>;

impl XError {
    pub fn table_error<S: Into<String>>(msg: S) -> Self {
        Self { kind: box XErrorKind::TableError(msg.into()), path: None }
    }
}
