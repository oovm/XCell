use super::*;

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
        Self { min: min.into(), max: max.into(), default: Default::default() }
    }
    pub fn clamp<I>(&self, int: I) -> BigDecimal
    where
        I: Into<BigDecimal>,
    {
        int.into()
    }
    pub fn parse_value(&self, cell: &DataType) -> Result<BigDecimal, XErrorKind> {
        match cell {
            DataType::Int(i) => Ok(self.clamp(*i)),
            DataType::Float(f) => match BigDecimal::from_f64(*f) {
                Some(o) => Ok(o),
                None => syntax_error(format!("{} 无法解析为 decimal 类型", f)),
            },
            DataType::String(s) => match BigDecimal::from_str(s) {
                Ok(o) => Ok(o),
                Err(_) => syntax_error(format!("{} 无法解析为 decimal 类型", s)),
            },
            DataType::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 decimal 类型", cell.to_string())),
        }
    }
    pub fn parse_f32(&self, cell: &DataType) -> Result<f32, XErrorKind> {
        match self.parse_value(cell) {
            Ok(o) => Ok(o.to_f32().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_f64(&self, cell: &DataType) -> Result<f64, XErrorKind> {
        match self.parse_value(cell) {
            Ok(o) => Ok(o.to_f64().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
}
