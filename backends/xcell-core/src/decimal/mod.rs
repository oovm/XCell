use serde::{Deserialize, Serialize};

use crate::{
    XResult,
    for_3rd::{BigDecimal, Data, FromPrimitive, ToPrimitive},
    XError, XErrorKind,
};

use crate::XCellValue;
mod kind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecimalKind {
    Float32,
    Float64,
    Decimal128,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DecimalDescription {
    pub kind: DecimalKind,
    pub min: BigDecimal,
    pub max: BigDecimal,
    pub default: BigDecimal,
}

impl DecimalDescription {
    pub fn range<A, B>(min: A, max: B) -> Self
    where
        A: Into<BigDecimal>,
        B: Into<BigDecimal>,
    {
        Self { kind: Default::default(), min: min.into(), max: max.into(), default: Default::default() }
    }
    pub fn clamp<I>(&self, int: I) -> BigDecimal
    where
        I: Into<BigDecimal>,
    {
        int.into()
    }
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        let dec = match cell {
            Data::Int(i) => *i as f64,
            Data::Float(f) => *f,
            Data::String(s) if s.trim().is_empty() => {
                return self.cast_decimal(self.default.clone());
            }
            Data::String(s) => {
                let min = self.min.to_f64();
                let max = self.max.to_f64();
                xcell_parser::parse_decimal_with_range(s.trim(), min, max)
                    .map_err(|e| XError::new(XErrorKind::SyntaxError { message: e.message }))?
            }
            Data::Empty => {
                return self.cast_decimal(self.default.clone());
            }
            _ => {
                return Err(XError::new(XErrorKind::SyntaxError {
                    message: format!("`{}` 无法解析为小数类型", cell),
                }));
            }
        };
        self.cast_decimal(BigDecimal::from_f64(dec).unwrap_or_default())
    }
    fn cast_decimal(&self, dec: BigDecimal) -> XResult<XCellValue> {
        match self.kind {
            DecimalKind::Float32 => match dec.to_f32() {
                Some(s) => Ok(XCellValue::Float32(s)),
                None => Err(XError::new(XErrorKind::SyntaxError {
                    message: format!("{} 无法转化为 f32 类型", dec),
                })),
            },
            DecimalKind::Float64 => match dec.to_f64() {
                Some(s) => Ok(XCellValue::Float64(s)),
                None => Err(XError::new(XErrorKind::SyntaxError {
                    message: format!("{} 无法转化为 f64 类型", dec),
                })),
            },
            DecimalKind::Decimal128 => match dec.to_f64() {
                Some(s) => Ok(XCellValue::Float64(s)),
                None => Err(XError::new(XErrorKind::SyntaxError {
                    message: format!("{} 无法转化为 d128 类型", dec),
                })),
            },
        }
    }
}
