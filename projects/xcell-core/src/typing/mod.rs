use std::str::FromStr;

use calamine::DataType;
use serde::{Deserialize, Serialize};

use crate::*;

pub(crate) mod boolean;

#[derive(Debug, Clone)]
pub enum XCellTyped {
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

#[derive(Debug, Default, Clone)]
pub struct CustomDescription {
    name: String,
}

impl FromStr for XCellTyped {
    type Err = XError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s.to_ascii_lowercase().as_str() {
            "str" | "string" => Self::String,
            "languageid" | "language" | "languagestring" => Self::LanguageID,
            "bool" | "boolean" => Self::Boolean(Default::default()),
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
            _ => Self::Custom(CustomDescription { name: s.to_string() }),
        };
        Ok(out)
    }
}

impl From<&DataType> for XCellTyped {
    // noinspection SpellCheckingInspection
    fn from(data: &DataType) -> Self {
        let s = data.to_string();
        todo!()
    }
}

impl XCellTyped {
    pub fn parse_cell(&self, cell: &DataType) -> Result<XCellTyped, XErrorKind> {
        match self {
            XCellTyped::Boolean(b) => match b.parse_cell(cell) {
                Ok(o) => Ok(XCellValue::Boolean(o)),
                Err(e) => Err(XErrorKind::TypeMismatch { except: XCellTyped::Boolean(Default::default()), current: e }),
            },
            XCellTyped::Integer8 => {
                todo!()
            }
            XCellTyped::Integer16 => {
                todo!()
            }
            XCellTyped::Integer32 => {
                todo!()
            }
            XCellTyped::Integer64 => {
                todo!()
            }
            XCellTyped::Unsigned8 => {
                todo!()
            }
            XCellTyped::Unsigned16 => {
                todo!()
            }
            XCellTyped::Unsigned32 => {
                todo!()
            }
            XCellTyped::Unsigned64 => {
                todo!()
            }
            XCellTyped::Float32 => {
                todo!()
            }
            XCellTyped::Float64 => {
                todo!()
            }
            XCellTyped::Float128 => {
                todo!()
            }
            XCellTyped::String => {
                todo!()
            }
            XCellTyped::LanguageID => {
                todo!()
            }
            XCellTyped::Datetime => {
                todo!()
            }
            XCellTyped::Color => {
                todo!()
            }
            XCellTyped::Custom(_) => {
                todo!()
            }
        }
        todo!()
    }
}

impl XCellTyped {}

impl XCellTable {
    pub fn parse_color(&mut self, cell: &DataType) -> bool {
        todo!()
    }

    fn type_mismatch<T>(&mut self, x: u32, y: u32, except: XCellTyped, current: XCellTyped, default: T) -> T {
        self.errors.push(XError::type_mismatch(x, y, except, current, &self.path));
        return default;
    }
}
