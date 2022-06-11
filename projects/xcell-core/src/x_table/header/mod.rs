use crate::typing::IntegerKind;

use super::*;

impl XCellHeaders {
    pub fn new<T, I>(i: I) -> Self
    where
        I: IntoIterator<Item = T>,
        XCellHeader: From<T>,
    {
        Self { kind: Default::default(), inner: Vec::from_iter(i.into_iter().map(XCellHeader::from)) }
    }
    pub fn with_kind(mut self, kind: XCellKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn check_enumerate(mut self) -> Self {
        self.try_enumerate();
        self
    }
    fn try_enumerate(&mut self) -> Option<()> {
        match self.inner.get(0)?.typing.is_enumerate() {
            true => self.kind = XCellKind::Enumerate,
            false => return None,
        }
        let kind = self.inner.get(1)?.typing.as_integer()?.kind;
        let a = self.inner.get_mut(0)?.typing.mut_enumerate()?;
        a.integer = kind;
        Some(())
    }
}

impl XCellHeader {
    pub fn parse_cell(&self, row: &[DataType]) -> Result<XCellValue, XErrorKind> {
        match row.get(self.column) {
            Some(cell) => self.typing.parse_cell(cell),
            None => {
                todo!()
            }
        }
    }
}

impl XCellTyped {
    pub fn parse_cell(&self, cell: &DataType) -> Result<XCellValue, XErrorKind> {
        match self {
            XCellTyped::Boolean(typing) => typing.parse_cell(cell).map(XCellValue::Boolean),
            XCellTyped::Integer(typing) => match typing.kind {
                IntegerKind::Integer8 => typing.parse_i8(cell).map(XCellValue::Integer8),
                IntegerKind::Integer16 => typing.parse_i16(cell).map(XCellValue::Integer16),
                IntegerKind::Integer32 => typing.parse_i32(cell).map(XCellValue::Integer32),
                IntegerKind::Integer64 => typing.parse_i64(cell).map(XCellValue::Integer64),
                IntegerKind::Unsigned8 => typing.parse_u8(cell).map(XCellValue::Unsigned8),
                IntegerKind::Unsigned16 => typing.parse_u16(cell).map(XCellValue::Unsigned16),
                IntegerKind::Unsigned32 => typing.parse_u32(cell).map(XCellValue::Unsigned32),
                IntegerKind::Unsigned64 => typing.parse_u64(cell).map(XCellValue::Unsigned64),
            },
            XCellTyped::Float32(typing) => typing.parse_f32(cell).map(XCellValue::Float32),
            XCellTyped::Float64(typing) => typing.parse_f64(cell).map(XCellValue::Float64),
            XCellTyped::Decimal128(typing) => typing.parse_f64(cell).map(XCellValue::Float64),
            XCellTyped::String(typing) => typing.parse_cell(cell).map(XCellValue::String),
            XCellTyped::Time(typing) => typing.parse_cell(cell).map(XCellValue::Time),
            XCellTyped::Color(typing) => typing.parse_cell(cell).map(XCellValue::Color),
            XCellTyped::Enumerate(typing) => typing.parse_cell(cell).map(XCellValue::String),
            XCellTyped::Custom(typing) => typing.parse_cell(cell).map(XCellValue::String),
        }
    }
}
impl Deref for XCellHeaders {
    type Target = [XCellHeader];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
