use xcell_errors::{for_3rd::DataType, XError, XErrorKind, XResult};

#[macro_export]
macro_rules! default_deserialize {
    ($($t:ty),*) => {
        $(
            impl<'de> Deserialize<'de> for $t {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_any(Self::default())
                }
            }
        )*
    };
}

pub fn type_mismatch<T, S>(except: S, cell: &DataType) -> XResult<T>
where
    S: Into<String>,
{
    let kind = XErrorKind::TypeMismatch { except: except.into(), current: cell.to_string() };
    Err(XError::new(kind))
}

pub fn syntax_error<T, A>(msg: A) -> XResult<T>
where
    A: Into<String>,
{
    let kind = XErrorKind::SyntaxError { message: msg.into() };
    Err(XError::new(kind))
}
