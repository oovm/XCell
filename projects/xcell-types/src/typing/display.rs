use super::*;

impl Debug for XCellTyped {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XCellTyped::Boolean(v) => Debug::fmt(v, f),
            XCellTyped::Integer(v) => Debug::fmt(v, f),
            XCellTyped::Decimal(v) => Debug::fmt(v, f),
            XCellTyped::String(v) => Debug::fmt(v, f),
            XCellTyped::Time(v) => Debug::fmt(v, f),
            XCellTyped::Color(v) => Debug::fmt(v, f),
            XCellTyped::Enumerate(v) => Debug::fmt(v, f),
            XCellTyped::Array(v) => Debug::fmt(v, f),
            XCellTyped::Vector(v) => Debug::fmt(v, f),
            XCellTyped::Custom(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for XCellTyped {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
