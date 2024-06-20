use super::*;

impl Default for IntegerKind {
    fn default() -> Self {
        IntegerKind::Unsigned32
    }
}

#[cfg(feature = "typescript")]
impl IntegerKind {
    /// 返回当前整数类型对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> &'static str {
        match self {
            IntegerKind::Integer8 => "number",
            IntegerKind::Integer16 => "number",
            IntegerKind::Integer32 => "number",
            IntegerKind::Integer64 => "number",
            IntegerKind::Unsigned8 => "number",
            IntegerKind::Unsigned16 => "number",
            IntegerKind::Unsigned32 => "number",
            IntegerKind::Unsigned64 => "number",
        }
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
    pub fn cast_integer(&self, input: BigInt) -> XCellValue {
        match self {
            IntegerKind::Integer8 => XCellValue::Integer8(input.to_i8().unwrap_or_default()),
            IntegerKind::Integer16 => XCellValue::Integer16(input.to_i16().unwrap_or_default()),
            IntegerKind::Integer32 => XCellValue::Integer32(input.to_i32().unwrap_or_default()),
            IntegerKind::Integer64 => XCellValue::Integer64(input.to_i64().unwrap_or_default()),
            IntegerKind::Unsigned8 => XCellValue::Unsigned8(input.to_u8().unwrap_or_default()),
            IntegerKind::Unsigned16 => XCellValue::Unsigned16(input.to_u16().unwrap_or_default()),
            IntegerKind::Unsigned32 => XCellValue::Unsigned32(input.to_u32().unwrap_or_default()),
            IntegerKind::Unsigned64 => XCellValue::Unsigned64(input.to_u64().unwrap_or_default()),
        }
    }
}
