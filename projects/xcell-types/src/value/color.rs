use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ColorDescription {
    pub alpha: bool,
    pub default: Color,
}

impl ColorDescription {
    pub fn gray<F>(color: F) -> Color
    where
        F: Into<f64>,
    {
        let c = color.into();
        Color::new(c, c, c, c)
    }
    pub fn parse<T>(&self, input: T) -> XResult<Color>
    where
        T: AsRef<str>,
    {
        Ok(Color::from_str(input.as_ref())?)
    }

    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValueKind> {
        self.parse_value(cell).map(XCellValueKind::Color)
    }
    fn parse_value(&self, cell: &DataType) -> XResult<Color> {
        match cell {
            DataType::Int(i) => Ok(Self::gray(*i as f64)),
            DataType::Float(f) => Ok(Self::gray(*f)),
            DataType::String(s) => self.parse(s),
            DataType::Empty => Ok(self.default.clone()),
            DataType::Error(e) => syntax_error(format!("未知错误 {e}")),
            _ => type_mismatch("Color", cell),
        }
    }
}
