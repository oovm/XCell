use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BooleanDescription {
    accept: Vec<String>,
    reject: Vec<String>,
    default: bool,
}

impl BooleanDescription {
    pub fn parse_cell(&self, cell: &DataType) -> Result<bool, XCellTyped> {
        match cell {
            DataType::Int(_) => Err(XCellTyped::Integer32),
            DataType::Float(_) => Err(XCellTyped::Float32),
            DataType::String(s) => {
                if self.accept.contains(s) {
                    Ok(true)
                }
                else if self.reject.contains(s) {
                    Ok(false)
                }
                else {
                    Err(XCellTyped::String)
                }
            }
            DataType::Bool(v) => Ok(*v),
            DataType::DateTime(_) => Err(XCellTyped::Datetime),
            DataType::Error(e) => Err(XCellTyped::Custom(CustomDescription { name: e.to_string() })),
            DataType::Empty => Ok(self.default),
        }
    }
}
