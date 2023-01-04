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
    utils::{syntax_error, type_mismatch},
    XCellTyped,
};

pub mod color;
pub mod convert;
mod display;
pub mod time;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCellValue {
    /// 实际储存的值
    pub kind: XCellValueKind,
    /// 从左往右数第几个, 0-index
    pub x: usize,
    /// 从上往下数第几个, 0-index
    pub y: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XCellValueKind {
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
    // Time(DateTime),
    Vector(Vec<XCellValueKind>),
    Enumerate(String),
}

impl Default for XCellValueKind {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

impl XCellValueKind {
    pub fn link_enumerate(&mut self, typing: &XCellTyped) -> XResult<()> {
        let (value, map) = match (&self, typing) {
            (XCellValueKind::Enumerate(v), XCellTyped::Enumerate(t)) => (v, t),
            _ => return Ok(()),
        };
        let default = map.mapping.get(&map.default).cloned().unwrap_or_default();
        let value = map.mapping.get(value).cloned();
        match value {
            Some(s) => {
                *self = map.integer.cast_integer(s);
            }
            None => {
                *self = map.integer.cast_integer(default);
            }
        }
        Ok(())
    }
}
