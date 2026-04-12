use crate::{
    XResult,
    for_3rd::{BigInt, Data, ToPrimitive},
    XError, XErrorKind,
};
use serde::{Deserialize, Serialize};

use crate::{typing::XCellTyped, value::XCellValue};

mod kind;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IntegerDescription {
    pub kind: IntegerKind,
    pub min: BigInt,
    pub max: BigInt,
    pub default: BigInt,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum IntegerKind {
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
}

impl IntegerDescription {
    pub fn range<A, B>(min: A, max: B, kind: IntegerKind) -> Self
    where
        A: Into<BigInt>,
        B: Into<BigInt>,
    {
        IntegerDescription { kind, min: min.into(), max: max.into(), default: Default::default() }
    }
    pub fn clamp<I>(&self, int: I) -> BigInt
    where
        I: Into<BigInt>,
    {
        int.into().clamp(self.min.clone(), self.max.clone())
    }
    pub fn parse_value(&self, cell: &Data) -> XResult<BigInt> {
        let int = match cell {
            Data::Int(i) => *i,
            Data::Float(f) => {
                let truncated = f.trunc() as i64;
                if truncated as f64 != *f {
                    return Err(XError::new(XErrorKind::SyntaxError {
                        message: format!("浮点数 `{}` 无法精确转换为整数类型", f),
                    }));
                }
                truncated
            }
            Data::String(s) if s.trim().is_empty() => {
                return Ok(self.default.clone());
            }
            Data::String(s) => {
                let min = self.min.to_i64();
                let max = self.max.to_i64();
                xcell_parser::parse_integer_with_range(s.trim(), min, max)
                    .map_err(|e| XError::new(XErrorKind::SyntaxError { message: e.message }))?
            }
            Data::Empty => {
                return Ok(self.default.clone());
            }
            _ => {
                return Err(XError::new(XErrorKind::SyntaxError {
                    message: format!("`{}` 无法解析为整数类型", cell),
                }));
            }
        };
        Ok(self.clamp(int))
    }
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        let value = self.parse_value(cell)?;
        Ok(self.kind.cast_integer(value))
    }
}

impl XCellTyped {
    pub fn as_integer(&self) -> Option<&IntegerDescription> {
        match self {
            XCellTyped::Integer(e) => Some(e),
            _ => None,
        }
    }
    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }
}

impl From<IntegerDescription> for XCellTyped {
    fn from(desc: IntegerDescription) -> Self {
        XCellTyped::Integer(Box::new(desc))
    }
}
