use serde::{Deserialize, Serialize};

use xcell_errors::{
    for_3rd::{BigDecimal, DataType, FromPrimitive, ToPrimitive},
    XResult,
};

use crate::{utils::syntax_error, XCellValue};
use std::ops::Deref;
mod kind;
mod parse_cell;

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
}
