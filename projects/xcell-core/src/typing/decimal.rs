use num::FromPrimitive;

use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DecimalDescription {
    pub min: BigInt,
    pub max: BigInt,
    pub default: BigInt,
}

impl DecimalDescription {
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
    pub fn parse_f32(&self, cell: &DataType) -> Result<f32, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_i32().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_f64(&self, cell: &DataType) -> Result<f64, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_i64().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_d128(&self, cell: &DataType) -> Result<f64, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_i64().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
}
