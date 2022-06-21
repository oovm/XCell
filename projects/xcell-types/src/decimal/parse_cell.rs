use super::*;

impl DecimalDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {}

    pub fn parse_value(&self, cell: &DataType) -> XResult<BigDecimal> {
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
    fn parse_f32(&self, cell: &DataType) -> XResult<f32> {
        match self.parse_value(cell) {
            Ok(o) => Ok(o.to_f32().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    fn parse_f64(&self, cell: &DataType) -> XResult<f64> {
        match self.parse_value(cell) {
            Ok(o) => Ok(o.to_f64().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
}
