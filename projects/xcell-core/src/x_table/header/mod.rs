use xcell_errors::XResult;
use xcell_types::{EnumerateDescription, XCellValue, XTableKind};

use super::*;

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
        if XTableKind::Enumerate == self.kind {
            let _: Option<()> = try {
                // 收集枚举类型
                let lhs = self.inner.get(0)?.typing.as_custom()?;
                let rhs = self.inner.get(1)?.typing.as_integer()?.kind;
                self.inner.get_mut(0)?.typing = EnumerateDescription::custom(lhs, rhs).into();
            };
        }
        self
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
