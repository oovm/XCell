use super::*;

impl ArrayDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<Vec<f64>> {
        match cell {
            DataType::Int(i) => Ok(vec![*i as f64]),
            DataType::Float(f) => Ok(vec![*f as f64]),
            DataType::String(s) => {
                let mut out = vec![];
                for item in s.split(',').map(|s| s.trim()) {
                    match f64::from_str(item) {
                        Ok(o) => out.push(o),
                        Err(_) => return syntax_error(format!("{} 无法解析为 decimal 类型", item.to_string())),
                    }
                }
                Ok(out)
            }
            DataType::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 decimal 类型", cell.to_string())),
        }
    }
    pub fn parse_vec2(&self, cell: &DataType) -> XResult<XCellValue> {
        todo!()
    }
    pub fn parse_vec3(&self, cell: &DataType) -> XResult<XCellValue> {
        todo!()
    }
    pub fn parse_vec4(&self, cell: &DataType) -> XResult<XCellValue> {
        todo!()
    }
}
