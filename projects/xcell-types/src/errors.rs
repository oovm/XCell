pub fn type_mismatch<T, A>(this: &A, cell: &DataType) -> Result<T, XErrorKind>
where
    A: Clone + Into<XCellTyped>,
{
    Err(XErrorKind::TypeMismatch { except: this.clone().into(), current: cell.clone() })
}

pub fn syntax_error<T, A>(msg: A) -> XResult<T>
where
    A: Into<String>,
{
    Err(XError { kind: box XErrorKind::SyntaxError(msg.into()), path: None, position: None })
}
