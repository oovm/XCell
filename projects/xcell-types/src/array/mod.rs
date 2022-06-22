use crate::{errors::syntax_error, XCellValue};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use xcell_errors::{for_3rd::DataType, XResult};

mod parse_cell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayKind {
    Vector2,
    Vector3,
    Vector4,
    Color4,
    Quaternion4,
}

impl Default for ArrayKind {
    fn default() -> Self {
        Self::Vector3
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ArrayDescription {
    pub kind: ArrayKind,
    pub default: Vec<f64>,
}

impl ArrayDescription {
    pub fn new(kind: ArrayKind) -> Self {
        Self { kind, default: vec![] }
    }
}
