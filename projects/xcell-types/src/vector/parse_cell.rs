use super::*;

impl VectorDescription {
    pub fn parse<T>(&self, input: T) -> XResult<XCellValue>
    where
        T: AsRef<str>,
    {
        todo!()
    }
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {
        todo!()
        // match cell {
        //     DataType::Int(i) => Ok(Self::gray(*i as f64)),
        //     DataType::Float(f) => Ok(Self::gray(*f)),
        //     DataType::String(s) => self.parse(s),
        //     DataType::Empty => Ok(self.default.clone()),
        //     DataType::Error(e) => syntax_error(format!("未知错误 {e}")),
        //     _ => type_mismatch(self, cell),
        // }
    }
}
