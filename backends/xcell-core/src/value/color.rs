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
        let c = color.into() as f32;
        Color::new(c, c, c, c)
    }
    pub fn parse<T>(&self, input: T) -> XResult<Color>
    where
        T: AsRef<str>,
    {
        Ok(Color::from_str(input.as_ref())?)
    }

    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        self.parse_value(cell).map(XCellValue::Color)
    }
    fn parse_value(&self, cell: &Data) -> XResult<Color> {
        match cell {
            Data::Int(i) => Ok(Self::gray(*i as f64)),
            Data::Float(f) => Ok(Self::gray(*f)),
            Data::String(s) => self.parse(s),
            Data::Empty => Ok(self.default.clone()),
            Data::Error(e) => syntax_error(format!("未知错误 {e}")),
            _ => type_mismatch("Color", cell),
        }
    }
}
