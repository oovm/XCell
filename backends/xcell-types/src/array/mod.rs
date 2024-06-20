use std::{ops::Deref, str::FromStr};

use crate::XResult;
use serde::{Deserialize, Serialize};

use crate::{XCellTyped, XCellValue, utils::syntax_error};

mod kind;
mod parse_cell;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayKind {
    Vector2,
    Vector3,
    Vector4,
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

    #[cfg(feature = "typescript")]
    /// 返回当前数组描述对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> &'static str {
        self.kind.as_typescript_type()
    }
}
