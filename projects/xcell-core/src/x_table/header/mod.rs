use super::*;
use xcell_errors::XResult;
use xcell_types::{XCellTyped, XCellValue, XTableKind};

impl XCellHeaders {
    pub fn new<T, I>(i: I) -> Self
    where
        I: IntoIterator<Item = T>,
        XCellHeader: From<T>,
    {
        Self { kind: Default::default(), inner: Vec::from_iter(i.into_iter().map(XCellHeader::from)) }
    }
    pub fn with_kind(mut self, kind: XTableKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn check_enumerate(mut self) -> Self {
        self.try_enumerate();
        self
    }
    fn try_enumerate(&mut self) -> Option<()> {
        match self.inner.get(0)?.typing.is_enumerate() {
            true => self.kind = XTableKind::Enumerate,
            false => return None,
        }
        let kind = self.inner.get(1)?.typing.as_integer()?.kind;
        let a = self.inner.get_mut(0)?.typing.mut_enumerate()?;
        a.integer = kind;
        Some(())
    }
}

impl XCellHeader {
    pub fn parse_cell(&self, row: &[DataType]) -> XResult<XCellValue> {
        match row.get(self.column) {
            Some(cell) => self.typing.parse_cell(cell),
            None => {
                todo!()
            }
        }
    }
}

impl Deref for XCellHeaders {
    type Target = [XCellHeader];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
