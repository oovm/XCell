use std::{ops::Deref, str::FromStr, time::SystemTime};

use bigdecimal::BigDecimal;
use calamine::DataType;
use chrono::Utc;
use csscolorparser::Color;
use num::{BigInt, FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

pub use crate::{
    decimal::{DecimalDescription, DecimalKind},
    integer::{IntegerDescription, IntegerKind},
};
use crate::{DateTime, XError, XErrorKind, XResult};

pub use self::{
    array::{ArrayDescription, ArrayKind},
    boolean::BooleanDescription,
    color::ColorDescription,
    custom::CustomDescription,
    enumerate::EnumerateDescription,
    string::StringDescription,
    time::TimeDescription,
    vector::VectorDescription,
};

mod array;
mod boolean;
mod color;
mod custom;
mod display;
mod enumerate;
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
