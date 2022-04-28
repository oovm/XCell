use super::*;

#[derive(Debug, Clone)]
pub enum XCellType {
    Boolean,
    String,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
}

impl From<&DataType> for XCellType {
    fn from(data: &DataType) -> Self {
        let s = data.to_string();
        match s.to_ascii_lowercase().as_str() {
            "bool" | "boolean" => Self::Boolean,
            // int
            "byte" | "i8" => Self::Integer8,
            "short" | "i16" => Self::Integer16,
            "int" | "i32" => Self::Integer32,
            "long" | "i64" => Self::Integer64,
            // unsigned
            "sbyte" | "u8" => Self::Unsigned8,
            "ushort" | "u16" => Self::Unsigned16,
            "uint" | "u32" => Self::Unsigned32,
            "ulong" | "u64" => Self::Unsigned64,

            _ => Self::String,
        }
    }
}
