use std::str::FromStr;

use calamine::DataType;
use csscolorparser::Color;
use num::BigInt;
use serde::{Deserialize, Serialize};

use crate::{XCellTable, XError, XErrorKind};

pub use self::{boolean::BooleanDescription, color::ColorDescription, integer::IntegerDescription};

mod boolean;
mod color;
mod integer;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XCellTyped {
    Boolean(BooleanDescription),
    Integer8(IntegerDescription),
    Integer16(IntegerDescription),
    Integer32(IntegerDescription),
    Integer64(IntegerDescription),
    Unsigned8(IntegerDescription),
    Unsigned16(IntegerDescription),
    Unsigned32(IntegerDescription),
    Unsigned64(IntegerDescription),
    Float32,
    Float64,
    Float128,
    String,
    LanguageID,
    Datetime,
    Color(ColorDescription),
    Custom(CustomDescription),
}

fn type_mismatch<T, A, B>(this: &A, cell: &B) -> Result<T, XErrorKind>
where
    A: Clone + Into<XCellTyped>,
    B: Clone,
{
    Err(XErrorKind::TypeMismatch { except: this.clone().into(), current: cell.clone() })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomDescription {
    name: String,
}

impl Default for XCellTyped {
    fn default() -> Self {
        Self::String
    }
}

impl FromStr for XCellTyped {
    type Err = XError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s.to_ascii_lowercase().as_str() {
            "str" | "string" => Self::String,
            "language" | "languagestring" | "languageid" => Self::LanguageID,
            "bool" | "boolean" => Self::Boolean(Default::default()),
            // int
            "byte" | "i8" => Self::Integer8(Default::default()),
            "short" | "i16" => Self::Integer16(Default::default()),
            "int" | "i32" => Self::Integer32(Default::default()),
            "long" | "i64" => Self::Integer64(Default::default()),
            // unsigned
            "sbyte" | "u8" => Self::Unsigned8(Default::default()),
            "ushort" | "u16" => Self::Unsigned16(Default::default()),
            "uint" | "u32" => Self::Unsigned32(Default::default()),
            "ulong" | "u64" => Self::Unsigned64(Default::default()),
            // float
            "float" | "f32" => Self::Float32,
            "double" | "f64" => Self::Float64,
            "decimal" | "f128" => Self::Float128,
            "color" => Self::Color(Default::default()),
            _ => Self::Custom(CustomDescription { name: s.to_string() }),
        };
        Ok(out)
    }
}

impl From<&DataType> for XCellTyped {
    fn from(data: &DataType) -> Self {
        XCellTyped::from_str(&data.to_string()).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum XCellValue {
    Boolean(bool),
    Integer(BigInt),
    Color(Color),
}

impl XCellTyped {}

impl XCellTable {
    pub fn parse_color(&mut self, cell: &DataType) -> bool {
        match cell {
            DataType::Int(_) => {}
            DataType::Float(_) => {}
            DataType::String(_) => {}
            DataType::Bool(_) => {}
            DataType::DateTime(_) => {}
            DataType::Error(_) => {}
            DataType::Empty => {}
        }

        todo!()
    }
}
