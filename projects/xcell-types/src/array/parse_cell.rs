use super::*;

impl ArrayDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<Vec<f64>> {
        match cell {
            DataType::Int(i) => Ok(vec![*i as f64]),
            DataType::Float(f) => Ok(vec![*f]),
            DataType::String(s) => {
                let mut out = vec![];
                for item in s.split(',').map(|s| s.trim()) {
                    match f64::from_str(item) {
                        Ok(o) => out.push(o),
                        Err(_) => return syntax_error(format!("{} 无法解析为 decimal 类型", item)),
                    }
                }
                Ok(out)
            }
            DataType::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 decimal 类型", cell)),
        }
    }
    pub fn parse_vec2(&self, cell: &DataType) -> XResult<XCellValue> {
        let vec = self.parse_cell(cell)?;
        let v1 = vec.get(0).cloned().unwrap_or(0.0);
        let v2 = vec.get(1).cloned().unwrap_or(0.0);
        Ok(XCellValue::Vector2([v1 as f32, v2 as f32]))
    }
    pub fn parse_vec3(&self, cell: &DataType) -> XResult<XCellValue> {
        let vec = self.parse_cell(cell)?;
        let v1 = vec.get(0).cloned().unwrap_or(0.0);
        let v2 = vec.get(1).cloned().unwrap_or(0.0);
        let v3 = vec.get(2).cloned().unwrap_or(0.0);
        Ok(XCellValue::Vector3([v1 as f32, v2 as f32, v3 as f32]))
    }
    pub fn parse_vec4(&self, cell: &DataType) -> XResult<XCellValue> {
        let vec = self.parse_cell(cell)?;
        Ok(XCellValue::Vector4(view4(&vec)))
    }
    pub fn parse_color4(&self, cell: &DataType) -> XResult<XCellValue> {
        let vec = self.parse_cell(cell)?;
        Ok(XCellValue::Color4(view4(&vec)))
    }
    pub fn parse_quaternion4(&self, cell: &DataType) -> XResult<XCellValue> {
        let vec = self.parse_cell(cell)?;
        Ok(XCellValue::Quaternion4(view4(&vec)))
    }
}

fn view4(vec: &[f64]) -> [f32; 4] {
    let v1 = vec.get(0).cloned().unwrap_or(0.0);
    let v2 = vec.get(1).cloned().unwrap_or(0.0);
    let v3 = vec.get(2).cloned().unwrap_or(0.0);
    let v4 = vec.get(3).cloned().unwrap_or(0.0);
    [v1 as f32, v2 as f32, v3 as f32, v4 as f32]
}
