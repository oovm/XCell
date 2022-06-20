use std::{ops::Deref, str::FromStr, time::SystemTime};

use bigdecimal::BigDecimal;
use calamine::DataType;
use chrono::Utc;
use csscolorparser::Color;
use num::{BigInt, FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::{DateTime, XError, XErrorKind, XResult};

pub use self::{
    array::{ArrayDescription, ArrayKind},
    boolean::BooleanDescription,
    color::ColorDescription,
    custom::CustomDescription,
    decimal::{DecimalDescription, DecimalKind},
    enumerate::EnumerateDescription,
    integer::{IntegerDescription, IntegerKind},
    string::StringDescription,
    time::TimeDescription,
    vector::VectorDescription,
};

mod array;
mod boolean;
mod color;
mod custom;
mod decimal;
mod display;
mod enumerate;
mod integer;
mod parser;
mod string;
mod time;
mod vector;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XCellTyped {
    Boolean(Box<BooleanDescription>),
    Integer(Box<IntegerDescription>),
    Decimal(Box<DecimalDescription>),
    String(Box<StringDescription>),
    Time(Box<TimeDescription>),
    Color(Box<ColorDescription>),
    Array(Box<ArrayDescription>),
    Enumerate(Box<EnumerateDescription>),
    Custom(Box<CustomDescription>),
    Vector(Box<VectorDescription>),
}

fn type_mismatch<T, A>(this: &A, cell: &DataType) -> Result<T, XErrorKind>
where
    A: Clone + Into<XCellTyped>,
{
    Err(XErrorKind::TypeMismatch { except: this.clone().into(), current: cell.clone() })
}

fn syntax_error<T, A>(msg: A) -> XResult<T>
where
    A: Into<String>,
{
    Err(XError { kind: box XErrorKind::SyntaxError(msg.into()), path: None, position: None })
}

impl FromStr for XCellTyped {
    type Err = XError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s.to_ascii_lowercase().as_str() {
            "str" | "string" => Self::String(Default::default()),
            "language" | "languagestring" | "languageid" => Self::String(Default::default()),
            "bool" | "boolean" => Self::Boolean(Default::default()),
            // int
            "byte" | "i8" => Self::Integer(IntegerDescription::range(i8::MIN, i8::MAX, IntegerKind::Integer8)),
            "short" | "i16" => Self::Integer(IntegerDescription::range(i16::MIN, i16::MAX, IntegerKind::Integer16)),
            "int" | "i32" => Self::Integer(IntegerDescription::range(i32::MIN, i32::MAX, IntegerKind::Integer32)),
            "long" | "i64" => Self::Integer(IntegerDescription::range(i64::MIN, i64::MAX, IntegerKind::Integer64)),
            // unsigned
            "sbyte" | "u8" => Self::Integer(IntegerDescription::range(u8::MIN, u8::MAX, IntegerKind::Unsigned8)),
            "ushort" | "u16" => Self::Integer(IntegerDescription::range(u16::MIN, u16::MAX, IntegerKind::Unsigned16)),
            "uint" | "u32" => Self::Integer(IntegerDescription::range(u32::MIN, u32::MAX, IntegerKind::Unsigned32)),
            "ulong" | "u64" => Self::Integer(IntegerDescription::range(u64::MIN, u64::MAX, IntegerKind::Unsigned64)),
            // float
            "float" | "f32" => Self::Decimal(Default::default()),
            "double" | "f64" => Self::Float64(Default::default()),
            "decimal" | "d128" | "f128" => Self::Decimal128(Default::default()),
            // other
            "color" | "color32" => Self::Color(Default::default()),
            "date" | "time" | "datetime" => Self::Time(Default::default()),
            // array
            "v2" | "vec2" => Self::Custom(ArrayDescription::new(s)),
            "v3" | "vec3" => Self::Custom(ArrayDescription::new(s)),
            "v4" | "vec4" => Self::Custom(ArrayDescription::new(s)),
            "q4" | "quaternion" => Self::Custom(ArrayDescription::new(s)),
            "c4" | "color4" => Self::Custom(ArrayDescription::new(s)),
            // enum
            "enum" => Self::Enumerate(Default::default()),
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
    Float64(f64),
    Vector2([f32; 2]),
    Vector3([f32; 3]),
    Vector4([f32; 4]),
    Color4([f32; 4]),
    Quaternion4([f32; 4]),
    String(String),
    Color(Color),
    Time(DateTime),
    Custom(String),
    Vector(Vec<XCellValue>),
}
