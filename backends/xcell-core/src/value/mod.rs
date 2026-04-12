use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    XResult,
    for_3rd::{Color, Data, DateTime, TimeZone, Utc},
};

use crate::{
    XCellTyped,
    utils::{syntax_error, type_mismatch},
};

pub mod color;
pub mod document;
mod display;
pub mod time;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCellNode {
    /// 实际储存的值
    pub kind: XCellValue,
    /// 从左往右数第几个, 0-index
    pub x: usize,
    /// 从上往下数第几个, 0-index
    pub y: usize,
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
    Quaternion4([f32; 4]),
    String(String),
    Color(Color),
    // Time(DateTime),
    Vector(Vec<XCellValue>),
    Enumerate(String),
    /// 引用类型，存储引用 ID
    Reference(i64),
    /// 映射值
    Map(BTreeMap<String, XCellValue>),
    /// 可选值
    Optional(Option<Box<XCellValue>>),
}

impl Default for XCellValue {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

impl XCellValue {
    pub fn link_enumerate(&mut self, typing: &XCellTyped) -> XResult<()> {
        let (value, map) = match (&self, typing) {
            (XCellValue::Enumerate(v), XCellTyped::Enumerate(t)) => (v, t),
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

    /// 将 XCellValue 转换为 serde_json::Value，直接输出值而非 { type: value } 格式
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            XCellValue::Boolean(v) => serde_json::Value::Bool(*v),
            XCellValue::Integer8(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Integer16(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Integer32(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Integer64(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Unsigned8(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Unsigned16(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Unsigned32(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Unsigned64(v) => {
                if let Some(n) = serde_json::Number::from(*v).as_i64() {
                    serde_json::Value::Number(n.into())
                } else {
                    serde_json::Value::String(v.to_string())
                }
            }
            XCellValue::Float32(v) => {
                if let Some(n) = serde_json::Number::from_f64(*v as f64) {
                    serde_json::Value::Number(n)
                } else {
                    serde_json::Value::Null
                }
            }
            XCellValue::Float64(v) => {
                if let Some(n) = serde_json::Number::from_f64(*v) {
                    serde_json::Value::Number(n)
                } else {
                    serde_json::Value::Null
                }
            }
            XCellValue::Vector2(v) => serde_json::json!([v[0], v[1]]),
            XCellValue::Vector3(v) => serde_json::json!([v[0], v[1], v[2]]),
            XCellValue::Vector4(v) => serde_json::json!([v[0], v[1], v[2], v[3]]),
            XCellValue::Quaternion4(v) => serde_json::json!([v[0], v[1], v[2], v[3]]),
            XCellValue::String(v) => serde_json::Value::String(v.clone()),
            XCellValue::Color(v) => serde_json::Value::String(v.to_string()),
            XCellValue::Vector(v) => serde_json::Value::Array(v.iter().map(|x| x.to_json_value()).collect()),
            XCellValue::Enumerate(v) => serde_json::Value::String(v.clone()),
            XCellValue::Reference(v) => serde_json::Value::Number((*v).into()),
            XCellValue::Map(m) => {
                let mut map = serde_json::Map::new();
                for (k, v) in m {
                    map.insert(k.clone(), v.to_json_value());
                }
                serde_json::Value::Object(map)
            }
            XCellValue::Optional(v) => {
                match v {
                    Some(inner) => inner.to_json_value(),
                    None => serde_json::Value::Null,
                }
            }
        }
    }
}
