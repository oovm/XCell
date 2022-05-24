use num::{FromPrimitive, ToPrimitive};

use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IntegerDescription {
    pub min: BigInt,
    pub max: BigInt,
    pub default: BigInt,
}

impl IntegerDescription {
    pub fn range<A, B>(min: A, max: B) -> Self
    where
        A: Into<BigInt>,
        B: Into<BigInt>,
    {
        Self { min: min.into(), max: max.into(), default: Default::default() }
    }
    pub fn clamp<I>(&self, int: I) -> BigInt
    where
        I: Into<BigInt>,
    {
        int.into().clamp(self.min.clone(), self.max.clone())
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<BigInt, XErrorKind> {
        match cell {
            DataType::Int(i) => Ok(self.clamp(*i)),
            DataType::Float(f) => match BigInt::from_f64(*f) {
                Some(o) => Ok(o),
                None => syntax_error(format!("{} 无法解析为 int 类型", f)),
            },
            DataType::String(s) => match BigInt::from_str(s) {
                Ok(o) => Ok(o),
                Err(_) => syntax_error(format!("{} 无法解析为 int 类型", s)),
            },
            DataType::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 int 类型", cell.to_string())),
        }
    }
    pub fn parse_i8(&self, cell: &DataType) -> Result<i8, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_i8().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_i16(&self, cell: &DataType) -> Result<i16, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_i16().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_i32(&self, cell: &DataType) -> Result<i32, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_i32().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_i64(&self, cell: &DataType) -> Result<i64, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_i64().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_u8(&self, cell: &DataType) -> Result<u8, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_u8().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_u16(&self, cell: &DataType) -> Result<u16, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_u16().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_u32(&self, cell: &DataType) -> Result<u32, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_u32().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_u64(&self, cell: &DataType) -> Result<u64, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_u64().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
}
