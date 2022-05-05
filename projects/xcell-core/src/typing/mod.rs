use std::str::FromStr;

use calamine::DataType;

use crate::*;

mod boolean;

#[derive(Debug, Clone)]
pub enum XCellValue {
    Boolean(bool),
    Integer8(i8),
    Integer16(i16),
    Integer32(i8),
    Integer64(i8),
    Unsigned8(i8),
    Unsigned16(i8),
    Unsigned32(i8),
    Unsigned64(i8),
    Float32(i8),
    Float64(i8),
    Float128(i8),
    String(String),
    LanguageID(String),
    Datetime(i8),
    Color,
}

#[derive(Debug, Clone)]
pub enum XCellType {
    Boolean(BooleanDescription),
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Float32,
    Float64,
    Float128,
    String,
    LanguageID,
    Datetime,
    Color,
    Custom(CustomDescription),
}

#[derive(Debug, Clone)]
pub struct BooleanDescription {
    r#true: Vec<String>,
    r#false: Vec<String>,
    default: bool,
}

#[derive(Debug, Clone)]
pub struct CustomDescription {
    name: String,
}

impl BooleanDescription {
    pub fn parse_cell(&self, cell: &DataType) -> Result<bool, XCellType> {
        match cell {
            DataType::Int(_) => Err(XCellType::Integer32),
            DataType::Float(_) => Err(XCellType::Float32),
            DataType::String(s) => {
                if self.r#true.contains(s) {
                    Ok(true)
                }
                else if self.r#false.contains(s) {
                    Ok(false)
                }
                else {
                    Err(XCellType::String)
                }
            }
            DataType::Bool(v) => Ok(*v),
            DataType::DateTime(_) => Err(XCellType::Datetime),
            DataType::Error(e) => Err(XCellType::Custom(e.to_string())),
            DataType::Empty => Ok(self.default),
        }
    }
}

impl FromStr for XCellType {
    type Err = XError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s.to_ascii_lowercase().as_str() {
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
            _ => Self::Custom(s.to_string()),
        };
        Ok(out)
    }
}

impl From<&DataType> for XCellType {
    // noinspection SpellCheckingInspection
    fn from(data: &DataType) -> Self {
        let s = data.to_string();
    }
}

impl XCellType {
    pub fn parse_cell(&self, cell: &DataType) -> Result<XCellValue, XError> {
        match self {
            XCellType::Boolean(b) => match b.parse_cell(cell) {
                Ok(o) => {}
                Err(_) => {}
            },
            XCellType::Integer8 => {
                todo!()
            }
            XCellType::Integer16 => {
                todo!()
            }
            XCellType::Integer32 => {
                todo!()
            }
            XCellType::Integer64 => {
                todo!()
            }
            XCellType::Unsigned8 => {
                todo!()
            }
            XCellType::Unsigned16 => {
                todo!()
            }
            XCellType::Unsigned32 => {
                todo!()
            }
            XCellType::Unsigned64 => {
                todo!()
            }
            XCellType::Float32 => {
                todo!()
            }
            XCellType::Float64 => {
                todo!()
            }
            XCellType::Float128 => {
                todo!()
            }
            XCellType::String => {
                todo!()
            }
            XCellType::LanguageID => {
                todo!()
            }
            XCellType::Datetime => {
                todo!()
            }
            XCellType::Color => {
                todo!()
            }
            XCellType::Custom(_) => {
                todo!()
            }
        }
    }
}

impl XCellType {}

impl XCellTable {
    pub fn parse_color(&mut self, cell: &DataType) -> bool {
        todo!()
    }

    fn type_mismatch<T>(&mut self, x: u32, y: u32, except: XCellType, current: XCellType, default: T) -> T {
        self.errors.push(XError::type_mismatch(x, y, except, current, &self.path));
        return default;
    }
}
