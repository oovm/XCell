use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VectorDescription {
    pub alpha: bool,
    pub default: Color,
}

impl From<VectorDescription> for XCellTyped {
    fn from(value: VectorDescription) -> Self {
        Self::Color(value)
    }
}

impl VectorDescription {
    pub fn gray<F>(color: F) -> Color
        where
            F: Into<f64>,
    {
        let c = color.into();
        Color::new(c, c, c, c)
    }
    pub fn parse<T>(&self, input: T) -> Result<Color, XErrorKind>
        where
            T: AsRef<str>,
    {
        let s = input.as_ref();
        match Color::from_str(s) {
            Ok(o) => Ok(o),
            Err(_) => syntax_error(format!("{} 无法解析为 color 类型", s)),
        }
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<Color, XErrorKind> {
        match cell {
            DataType::Int(i) => Ok(Self::gray(*i as f64)),
            DataType::Float(f) => Ok(Self::gray(*f)),
            DataType::String(s) => self.parse(s),
            DataType::Empty => Ok(self.default.clone()),
            DataType::Error(e) => syntax_error(format!("未知错误 {e}")),
            _ => type_mismatch(self, cell),
        }
    }
}
