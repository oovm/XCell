use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use xcell_errors::{
    for_3rd::{Color, DataType, DateTime, NaiveDateTime, TimeZone, Utc},
    XResult,
};

use crate::{
    errors::{syntax_error, type_mismatch},
    XCellTyped,
};

pub mod boolean;
pub mod color;
pub mod convert;
pub mod custom;
mod display;
pub mod enumerate;
pub mod string;
pub mod time;

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