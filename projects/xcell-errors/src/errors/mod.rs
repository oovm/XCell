use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::{Path, PathBuf},
};

mod for_std;

#[derive(Debug, Default)]
pub struct XError {
    pub kind: Box<XErrorKind>,
    pub path: Option<PathBuf>,
    pub position: Option<(usize, usize)>,
    pub source: Option<Box<dyn Error>>,
}

#[derive(Debug)]
pub enum XErrorKind {
    IOError(String),
    SyntaxError { message: String },
    TableError(String),
    TypeMismatch { except: String, current: String },
    UnknownError,
}

impl Display for XError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self.kind {
            XErrorKind::IOError(v) => write!(f, "IO 异常: {v}")?,
            XErrorKind::SyntaxError { message } => write!(f, "解析错误: {}", message)?,
            XErrorKind::TableError(v) => write!(f, "表错误: {}", v)?,
            XErrorKind::TypeMismatch { except, current } => write!(f, "类型错误: 预期 `{}`, 实际 `{}`", except, current)?,
            XErrorKind::UnknownError => write!(f, "内部错误")?,
        }
        if let Some(s) = &self.path {
            write!(f, "\n{}", s.display())?;
            match self.position {
                Some((x, y)) => writeln!(f, " ({x} 行 {y} 列)")?,
                None => writeln!(f)?,
            }
        }
        Ok(())
    }
}

impl Error for XError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(s) => Some(s.as_ref()),
            None => None,
        }
    }
}

impl XError {
    pub fn new(kind: XErrorKind) -> Self {
        Self { kind: Box::new(kind), path: None, position: None, source: None }
    }

    pub fn with_path(mut self, path: &Path) -> Self {
        self.path = Some(path.to_path_buf());
        self
    }
    pub fn with_xy(mut self, x: usize, y: usize) -> Self {
        self.position = Some((x, y));
        self
    }
    pub fn table_error<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        let kind = XErrorKind::TableError(msg.into());
        Self { kind: Box::new(kind), path: None, position: None, source: None }
    }
}
