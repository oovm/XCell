use super::*;

impl From<ArrayDescription> for XCellTyped {
    fn from(value: ArrayDescription) -> Self {
        Self::Array(Box::new(value))
    }
}

impl ArrayDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValueKind> {
        match self.kind {
            ArrayKind::Vector2 => self.parse_vec2(cell),
            ArrayKind::Vector3 => self.parse_vec3(cell),
            ArrayKind::Vector4 => self.parse_vec4(cell),
            ArrayKind::Color4 => self.parse_color4(cell),
            ArrayKind::Quaternion4 => self.parse_quaternion4(cell),
        }
    }
    fn parse_value(&self, cell: &DataType) -> XResult<Vec<f64>> {
        match cell {
            DataType::Int(i) => Ok(vec![*i as f64]),
            DataType::Float(f) => Ok(vec![*f]),
            DataType::String(s) => {
                let mut out = vec![];
                for item in s.split(',').map(|s| s.trim()) {
                    if item.is_empty() {
                        continue;
                    }
                    match f64::from_str(item) {
                        Ok(o) => out.push(o),
                        Err(_) => return syntax_error(format!("{:?} 无法解析为 decimal 类型", item)),
                    }
                }
                Ok(out)
            }
            DataType::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 decimal 类型", cell)),
        }
    }
    fn parse_vec2(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let vec = self.parse_value(cell)?;
        Ok(XCellValueKind::Vector2(fill_array(&vec)))
    }
    fn parse_vec3(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let vec = self.parse_value(cell)?;
        Ok(XCellValueKind::Vector3(fill_array(&vec)))
    }
    fn parse_vec4(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let vec = self.parse_value(cell)?;
        Ok(XCellValueKind::Vector4(fill_array(&vec)))
    }
    fn parse_color4(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let vec = self.parse_value(cell)?;
        Ok(XCellValueKind::Color4(fill_array(&vec)))
    }
    fn parse_quaternion4(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let vec = self.parse_value(cell)?;
        Ok(XCellValueKind::Quaternion4(fill_array(&vec)))
    }
}

fn fill_array<const N: usize>(vec: &[f64]) -> [f32; N] {
    let mut out = [0.0; N];
    for (i, v) in vec.iter().enumerate() {
        if i >= N {
            break;
        }
        out[i] = *v as f32;
    }
    out
}
