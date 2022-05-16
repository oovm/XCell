use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IntegerDescription {
    pub min: BigInt,
    pub max: BigInt,
    pub default: BigInt,
}

impl IntegerDescription {
    pub fn clamp<I>(&self, int: I) -> BigInt
    where
        I: Into<BigInt>,
    {
        int.into().clamp(self.min.clone(), self.max.clone())
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<BigInt, XErrorKind> {
        match cell {
            DataType::Int(i) => Ok(self.clamp(i)),
            DataType::Float(f) => Ok(self.clamp(f)),
            DataType::String(s) => match BigInt::from_str(s) {
                Ok(o) => Ok(o),
                Err(e) => todo!(),
            },
            DataType::Bool(_) => todo!(),
            DataType::DateTime(_) => todo!(),
            DataType::Error(e) => todo!(),
            DataType::Empty => Ok(self.default.clone()),
        }
    }
}
