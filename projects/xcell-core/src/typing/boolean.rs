use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BooleanDescription {
    accept: Vec<String>,
    reject: Vec<String>,
    default: bool,
}

impl From<BooleanDescription> for XCellTyped {
    fn from(value: BooleanDescription) -> Self {
        Self::Boolean(value)
    }
}

impl BooleanDescription {
    pub fn parse_cell(&self, cell: &DataType) -> Result<bool, XErrorKind> {
        match cell {
            DataType::String(s) => {
                if self.accept.contains(s) {
                    Ok(true)
                }
                else if self.reject.contains(s) {
                    Ok(false)
                }
            }
            DataType::Bool(v) => Ok(*v),
            DataType::Empty => Ok(self.default),
            _ => type_mismatch(self, cell),
        }
    }
}
