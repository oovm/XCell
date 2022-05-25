use std::str::FromStr;

use calamine::DataType;
use csscolorparser::Color;
use num::{BigInt, FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use bigdecimal::BigDecimal;

use crate::{XCellTable, XError, XErrorKind};

pub use self::{
    boolean::BooleanDescription, color::ColorDescription, custom::CustomDescription, decimal::DecimalDescription,
    integer::IntegerDescription, string::StringDescription, time::TimeDescription,
};

mod boolean;
mod color;
mod custom;
mod decimal;
mod display;
mod integer;
mod string;
mod time;

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
    Float32(DecimalDescription),
    Float64(DecimalDescription),
    Decimal128(DecimalDescription),
    String(StringDescription),
    LanguageID(StringDescription),
    Datetime(TimeDescription),
    Color(ColorDescription),
    Custom(CustomDescription),
}

fn type_mismatch<T, A>(this: &A, cell: &DataType) -> Result<T, XErrorKind>
where
    A: Clone + Into<XCellTyped>,
{
    Err(XErrorKind::TypeMismatch { except: this.clone().into(), current: cell.clone() })
}

fn syntax_error<T, A>(msg: A) -> Result<T, XErrorKind>
where
    A: Into<String>,
{
    Err(XErrorKind::SyntaxError(msg.into()))
}

impl FromStr for XCellTyped {
    type Err = XError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s.to_ascii_lowercase().as_str() {
            "str" | "string" => Self::String(Default::default()),
            "language" | "languagestring" | "languageid" => Self::LanguageID(Default::default()),
            "bool" | "boolean" => Self::Boolean(Default::default()),
            // int
            "byte" | "i8" => Self::Integer8(IntegerDescription::range(i8::MIN, i8::MAX)),
            "short" | "i16" => Self::Integer16(IntegerDescription::range(i16::MIN, i16::MAX)),
            "int" | "i32" => Self::Integer32(IntegerDescription::range(i32::MIN, i32::MAX)),
            "long" | "i64" => Self::Integer64(IntegerDescription::range(i64::MIN, i64::MAX)),
            // unsigned
            "sbyte" | "u8" => Self::Unsigned8(IntegerDescription::range(u8::MIN, u8::MAX)),
            "ushort" | "u16" => Self::Unsigned16(IntegerDescription::range(u16::MIN, u16::MAX)),
            "uint" | "u32" => Self::Unsigned32(IntegerDescription::range(u32::MIN, u32::MAX)),
            "ulong" | "u64" => Self::Unsigned64(IntegerDescription::range(u64::MIN, u64::MAX)),
            // float
            "float" | "f32" => Self::Float32(Default::default()),
            "double" | "f64" => Self::Float64(Default::default()),
            "decimal" | "f128" => Self::Decimal128(Default::default()),
            "color" => Self::Color(Default::default()),
            _ => Self::Custom(CustomDescription::new(s)),
        };
        Ok(out)
    }
}

impl From<&DataType> for XCellTyped {
    fn from(data: &DataType) -> Self {
        XCellTyped::from_str(&data.to_string()).unwrap()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XCellValue {
    Boolean(bool),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Float32(f32),
    Float64(u64),
    String(String),
    Color(Color),
    Custom(String),
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
