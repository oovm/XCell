use std::{ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use xcell_errors::{
    for_3rd::{BigInt, DataType, FromPrimitive, ToPrimitive},
    XResult,
};

use crate::{errors::syntax_error, typing::XCellTyped, value::XCellValue};

mod kind;
mod parse_cell;

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
        Self { kind, min: min.into(), max: max.into(), default: Default::default() }
    }
    pub fn clamp<I>(&self, int: I) -> BigInt
    where
        I: Into<BigInt>,
    {
        int.into().clamp(self.min.clone(), self.max.clone())
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