use crate::{IntegerKind, XCellValue};

impl XCellValue {
    pub fn csharp() {}
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
    pub fn as_csharp_reader(&self) -> &'static str {
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
