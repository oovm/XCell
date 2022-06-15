use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayKind {
    Vector2,
    Vector3,
    Vector4,
    Color4,
    Quaternion4,
}

impl Default for ArrayKind {
    fn default() -> Self {
        Self::Vector3
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ArrayDescription {
    pub kind: ArrayKind,
    pub default: Vec<f64>,
}

impl ArrayDescription {
    pub fn new(kind: ArrayKind) -> Self {
        Self { kind, default: vec![] }
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<Vec<f64>, XErrorKind> {
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
    pub fn parse_vec2(&self, cell: &DataType) -> Result<[f32; 3], XErrorKind> {
        match self.parse_cell(cell) {
            Ok(o) => Ok(o.to_f32().unwrap_or_default()),
            Err(e) => Err(e),
        }
    }
    pub fn parse_v3(&self, cell: &DataType) -> Result<[f32; 4], XErrorKind> {
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
