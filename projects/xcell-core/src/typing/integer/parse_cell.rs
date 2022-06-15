use super::*;

impl IntegerDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<BigInt> {
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

    fn parse_value(&self, cell: &DataType) -> XResult<BigInt> {
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

    fn parse_i8(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_i8() {
            Some(s) => Ok(XCellValue::Integer8(s)),
            None => syntax_error(format!("{int} 无法转化为 i8 类型")),
        }
    }
    fn parse_i16(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_i16() {
            Some(s) => Ok(XCellValue::Integer16(s)),
            None => syntax_error(format!("{int} 无法转化为 i16 类型")),
        }
    }
    fn parse_i32(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_i32() {
            Some(s) => Ok(XCellValue::Integer32(s)),
            None => syntax_error(format!("{int} 无法转化为 i32 类型")),
        }
    }
    fn parse_i64(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_i64() {
            Some(s) => Ok(XCellValue::Integer64(s)),
            None => syntax_error(format!("{int} 无法转化为 i64 类型")),
        }
    }
    fn parse_u8(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_u8() {
            Some(s) => Ok(XCellValue::Unsigned8(s)),
            None => syntax_error(format!("{int} 无法转化为 u8 类型")),
        }
    }
    fn parse_u16(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_u16() {
            Some(s) => Ok(XCellValue::Unsigned16(s)),
            None => syntax_error(format!("{int} 无法转化为 u16 类型")),
        }
    }
    fn parse_u32(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_u32() {
            Some(s) => Ok(XCellValue::Unsigned32(s)),
            None => syntax_error(format!("{int} 无法转化为 u32 类型")),
        }
    }
    fn parse_u64(&self, cell: &DataType) -> XResult<XCellValue> {
        let int = self.parse_value(cell)?;
        match int.to_u64() {
            Some(s) => Ok(XCellValue::Unsigned64(s)),
            None => syntax_error(format!("{int} 无法转化为 u64 类型")),
        }
    }
}
