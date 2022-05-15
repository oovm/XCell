use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IntegerDescription {
    pub min: BigInt,
    pub max: BitInt,
    pub default: Color,
}

impl IntegerDescription {
    pub fn gray<F>(color: F) -> Color
    where
        F: Into<f64>,
    {
        let c = color.into();
        Color::new(c, c, c, c)
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<Color, XCellTyped> {
        match cell {
            DataType::Int(i) => Ok(Self::gray(*i as f64)),
            DataType::Float(f) => Ok(Self::gray(*f)),
            DataType::String(s) => match Color::from_str(s) {
                Ok(o) => Ok(o),
                Err(_) => Err(XCellTyped::String),
            },
            DataType::Bool(true) => Ok(Self::gray(1.0)),
            DataType::Bool(false) => Ok(Self::gray(0.0)),
            DataType::DateTime(_) => Err(XCellTyped::Datetime),
            DataType::Error(e) => Err(XCellTyped::Custom(CustomDescription { name: e.to_string() })),
            DataType::Empty => Ok(self.default.clone()),
        }
    }
}
