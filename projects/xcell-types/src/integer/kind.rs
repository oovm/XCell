use super::*;

impl Default for IntegerKind {
    fn default() -> Self {
        IntegerKind::Unsigned32
    }
}

impl Deref for IntegerDescription {
    type Target = IntegerKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl IntegerKind {
    pub fn unsigned(&self) -> bool {
        !self.signed()
    }
    pub fn signed(&self) -> bool {
        match self {
            IntegerKind::Integer8 => true,
            IntegerKind::Integer16 => true,
            IntegerKind::Integer32 => true,
            IntegerKind::Integer64 => true,
            IntegerKind::Unsigned8 => false,
            IntegerKind::Unsigned16 => false,
            IntegerKind::Unsigned32 => false,
            IntegerKind::Unsigned64 => false,
        }
    }
    pub fn size(&self) -> usize {
        match self {
            IntegerKind::Integer8 => 1,
            IntegerKind::Integer16 => 2,
            IntegerKind::Integer32 => 4,
            IntegerKind::Integer64 => 8,
            IntegerKind::Unsigned8 => 1,
            IntegerKind::Unsigned16 => 2,
            IntegerKind::Unsigned32 => 4,
            IntegerKind::Unsigned64 => 8,
        }
    }
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
