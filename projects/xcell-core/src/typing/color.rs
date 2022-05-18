use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ColorDescription {
    pub alpha: bool,
    pub default: Color,
}

impl From<ColorDescription> for XCellTyped {
    fn from(value: ColorDescription) -> Self {
        Self::Color(value)
    }
}

impl ColorDescription {
    pub fn gray<F>(color: F) -> Color
    where
        F: Into<f64>,
    {
        let c = color.into();
        Color::new(c, c, c, c)
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<Color, XErrorKind> {
        match cell {
            DataType::Int(i) => Ok(Self::gray(*i as f64)),
            DataType::Float(f) => Ok(Self::gray(*f)),
            DataType::String(s) => match Color::from_str(s) {
                Ok(o) => Ok(o),
                Err(_) => syntax_error(format!("{} 无法解析为 color 类型", s)),
            },
            DataType::Empty => Ok(self.default.clone()),
            _ => type_mismatch(self, cell),
        }
    }
}
