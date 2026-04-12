use std::ops::Deref;

use crate::XResult;
use serde::{Deserialize, Serialize};

use crate::{XCellTyped, XCellValue, utils::syntax_error, for_3rd::Data};

mod kind;

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
}

impl From<ArrayDescription> for XCellTyped {
    fn from(value: ArrayDescription) -> Self {
        Self::Array(Box::new(value))
    }
}

impl ArrayDescription {
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        let vec = self.parse_value(cell)?;
        match self.kind {
            ArrayKind::Vector2 => {
                let arr = fill_array(&vec);
                Ok(XCellValue::Vector2(arr))
            }
            ArrayKind::Vector3 => {
                let arr = fill_array(&vec);
                Ok(XCellValue::Vector3(arr))
            }
            ArrayKind::Vector4 => {
                let arr = fill_array(&vec);
                Ok(XCellValue::Vector4(arr))
            }
            ArrayKind::Quaternion4 => {
                let arr = fill_array(&vec);
                Ok(XCellValue::Quaternion4(arr))
            }
        }
    }

    fn expected_len(&self) -> usize {
        match self.kind {
            ArrayKind::Vector2 => 2,
            ArrayKind::Vector3 => 3,
            ArrayKind::Vector4 => 4,
            ArrayKind::Quaternion4 => 4,
        }
    }

    fn parse_value(&self, cell: &Data) -> XResult<Vec<f64>> {
        match cell {
            Data::Int(i) => Ok(vec![*i as f64]),
            Data::Float(f) => Ok(vec![*f]),
            Data::String(s) => {
                let s = s.trim();
                let expected_len = self.expected_len();
                xcell_parser::parse_vector_with_len(s, expected_len).map_err(|e| e.into())
            }
            Data::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 decimal 类型", cell)),
        }
    }
}

fn fill_array<const N: usize>(vec: &[f64]) -> [f32; N] {
    let mut out = [0.0; N];
    for (i, v) in vec.iter().enumerate() {
        if i >= N {
            break;
        }
        out[i] = *v as f32;
    }
    out
}
