use super::*;
use std::str::FromStr;

impl DecimalDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValueKind> {
        match self.kind {
            DecimalKind::Float32 => self.parse_f32(cell),
            DecimalKind::Float64 => self.parse_f64(cell),
            DecimalKind::Decimal128 => self.parse_d128(cell),
        }
    }
    fn parse_value(&self, cell: &DataType) -> XResult<BigDecimal> {
        match cell {
            DataType::Int(i) => Ok(self.clamp(*i)),
            DataType::Float(f) => match BigDecimal::from_f64(*f) {
                Some(o) => Ok(o),
                None => syntax_error(format!("DataType::Float {} 无法解析为 decimal 类型", f)),
            },
            DataType::String(s) if s.trim().is_empty() => Ok(self.default.clone()),
            DataType::String(s) => match BigDecimal::from_str(s) {
                Ok(o) => Ok(o),
                Err(_) => syntax_error(format!("DataType::String {} 无法解析为 decimal 类型", s)),
            },
            DataType::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 decimal 类型", cell)),
        }
    }
    fn parse_f32(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let dec = self.parse_value(cell)?;
        match dec.to_f32() {
            Some(s) => Ok(XCellValueKind::Float32(s)),
            None => syntax_error(format!("{dec} 无法转化为 f32 类型")),
        }
    }
    fn parse_f64(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let dec = self.parse_value(cell)?;
        match dec.to_f64() {
            Some(s) => Ok(XCellValueKind::Float64(s)),
            None => syntax_error(format!("{dec} 无法转化为 f64 类型")),
        }
    }
    fn parse_d128(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let dec = self.parse_value(cell)?;
        match dec.to_f64() {
            Some(s) => Ok(XCellValueKind::Float64(s)),
            None => syntax_error(format!("{dec} 无法转化为 d128 类型")),
        }
    }
}
