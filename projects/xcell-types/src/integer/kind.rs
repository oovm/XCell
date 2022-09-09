use super::*;

impl Default for IntegerKind {
    fn default() -> Self {
        IntegerKind::Unsigned32
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
    pub fn cast_integer(&self, input: BigInt) -> XCellValueKind {
        match self {
            IntegerKind::Integer8 => XCellValueKind::Integer8(input.to_i8().unwrap_or_default()),
            IntegerKind::Integer16 => XCellValueKind::Integer16(input.to_i16().unwrap_or_default()),
            IntegerKind::Integer32 => XCellValueKind::Integer32(input.to_i32().unwrap_or_default()),
            IntegerKind::Integer64 => XCellValueKind::Integer64(input.to_i64().unwrap_or_default()),
            IntegerKind::Unsigned8 => XCellValueKind::Unsigned8(input.to_u8().unwrap_or_default()),
            IntegerKind::Unsigned16 => XCellValueKind::Unsigned16(input.to_u16().unwrap_or_default()),
            IntegerKind::Unsigned32 => XCellValueKind::Unsigned32(input.to_u32().unwrap_or_default()),
            IntegerKind::Unsigned64 => XCellValueKind::Unsigned64(input.to_u64().unwrap_or_default()),
        }
    }
}
