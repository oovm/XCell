use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    pub fn range<A, B>(min: A, max: B) -> Self
    where
        A: Into<BigDecimal>,
        B: Into<BigDecimal>,
    {
        Self { kind: min.into(), max: max.into(), default: Default::default() }
    }
    pub fn clamp<I>(&self, int: I) -> BigDecimal
    where
        I: Into<BigDecimal>,
    {
        int.into()
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<BigDecimal, XErrorKind> {
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
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_f32().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_f64(&self, cell: &DataType) -> Result<f64, XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_f64().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
}
