use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};
use xcell_errors::{for_3rd::DataType, XResult};

use crate::{utils::syntax_error, XCellTyped, XCellValueKind};

mod kind;
mod parse_cell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayKind {
    Vector2,
    Vector3,
    Vector4,
    Color4,
    Quaternion4,
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
