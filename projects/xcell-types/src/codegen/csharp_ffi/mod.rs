use crate::{BooleanDescription, EnumerateDescription, IntegerKind, XCellTyped, XCellValue};

mod default;

impl XCellValue {
    pub fn csharp() {}
}

impl XCellTyped {
    pub fn as_csharp_type(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "bool".to_string(),
            XCellTyped::Integer(v) => v.as_csharp_type().to_string(),
            XCellTyped::Decimal(v) => v.as_csharp_type().to_string(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Time(_) => "DateTime".to_string(),
            XCellTyped::Color(_) => "Color32".to_string(),
            XCellTyped::Enumerate(v) => v.typing.to_owned(),
            XCellTyped::Array(v) => v.as_csharp_type().to_string(),
            XCellTyped::Vector(_) => {
                todo!()
            }
        }
    }
}

impl IntegerKind {
    pub fn as_csharp_type(&self) -> &'static str {
        match self {
            IntegerKind::Integer8 => "byte",
            IntegerKind::Integer16 => "short",
            IntegerKind::Integer32 => "int",
            IntegerKind::Integer64 => "long",
            IntegerKind::Unsigned8 => "sbyte",
            IntegerKind::Unsigned16 => "ushort",
            IntegerKind::Unsigned32 => "uint",
            IntegerKind::Unsigned64 => "ulong",
        }
    }
    pub fn as_csharp_reader_function(&self) -> &'static str {
        match self {
            IntegerKind::Integer8 => "ReadByte",
            IntegerKind::Integer16 => "ReadInt16",
            IntegerKind::Integer32 => "ReadInt32",
            IntegerKind::Integer64 => "ReadInt64",
            IntegerKind::Unsigned8 => "ReadSByte",
            IntegerKind::Unsigned16 => "ReadUInt16",
            IntegerKind::Unsigned32 => "ReadUInt32",
            IntegerKind::Unsigned64 => "ReadUInt64",
        }
    }
}

impl EnumerateDescription {
    pub fn as_csharp_reader(&self) -> String {
        format!("({}) r.{}()", self.typing, self.integer.as_csharp_reader_function())
    }
}
