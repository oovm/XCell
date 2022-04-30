use super::*;

impl From<&DataType> for XCellType {
    // noinspection SpellCheckingInspection
    fn from(data: &DataType) -> Self {
        let s = data.to_string();
        match s.to_ascii_lowercase().as_str() {
            "str" | "string" => Self::String,
            "languageid" | "language" | "languagestring" => Self::LanguageString,
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
            // float
            "float" | "f32" => Self::Float32,
            "double" | "f64" => Self::Float64,
            "decimal" | "f128" => Self::Float128,
            _ => Self::Custom(s),
        }
    }
}
