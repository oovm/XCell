use super::*;

impl From<&DataType> for XCellType {
    // noinspection SpellCheckingInspection
    fn from(data: &DataType) -> Self {
        let s = data.to_string();
        match s.to_ascii_lowercase().as_str() {
            "str" | "string" => Self::String,
            "languageid" | "language" | "languagestring" => Self::LanguageID,
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

impl XCellType {
    pub fn parse_cell(&self, _: &DataType) {
        match self {
            XCellType::Boolean => {}
            XCellType::Integer8 => {}
            XCellType::Integer16 => {}
            XCellType::Integer32 => {}
            XCellType::Integer64 => {}
            XCellType::Unsigned8 => {}
            XCellType::Unsigned16 => {}
            XCellType::Unsigned32 => {}
            XCellType::Unsigned64 => {}
            XCellType::Float32 => {}
            XCellType::Float64 => {}
            XCellType::Float128 => {}
            XCellType::String => {}
            XCellType::LanguageID => {}
            XCellType::Custom(_) => {}
            XCellType::Datetime => {}
        }
    }
}

impl XCellTable {
    pub fn parse_bool(&mut self, cell: &DataType) -> bool {
        let cfg = &self.config.typing.boolean;
        match cell {
            DataType::Int(_) => {
                self.errors.push(XError::type_mismatch(0, 0, XCellType::Boolean, XCellType::Integer32, &self.path));
                cfg.default
            }
            DataType::Float(_) => {
                self.errors.push(XError::type_mismatch(0, 0, XCellType::Boolean, XCellType::Float32, &self.path));
                cfg.default
            }
            DataType::String(s) => {
                if cfg.r#true.contains(s) {
                    true
                }
                else if cfg.r#false.contains(s) {
                    false
                }
                else {
                    self.errors.push(XError::type_mismatch(0, 0, XCellType::Boolean, XCellType::String, &self.path));
                    cfg.default
                }
            }
            DataType::Bool(v) => *v,
            DataType::DateTime(_) => {
                self.errors.push(XError::type_mismatch(0, 0, XCellType::Boolean, XCellType::Datetime, &self.path));
                cfg.default
            }
            DataType::Error(e) => {
                self.errors.push(XError::type_mismatch(0, 0, XCellType::Boolean, XCellType::Custom(e.to_string()), &self.path));
                cfg.default
            }
            DataType::Empty => cfg.default,
        }
    }
}
