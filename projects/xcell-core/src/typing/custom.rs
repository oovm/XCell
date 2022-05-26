use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomDescription {
    pub typing: String,
    pub default: String,
}

impl CustomDescription {
    pub fn new<S>(typing: S) -> Self
    where
        S: Into<String>,
    {
        Self { typing: typing.into(), default: "".to_string() }
    }
    pub fn parse_cell(&self, cell: &DataType) -> Result<String, XErrorKind> {
        match cell {
            DataType::Int(v) => Ok(v.to_string()),
            DataType::Float(v) => Ok(v.to_string()),
            DataType::String(v) => Ok(v.to_string()),
            DataType::Bool(v) => Ok(v.to_string()),
            DataType::DateTime(v) => Ok(v.to_string()),
            DataType::Empty => Ok(self.default.clone()),
            DataType::Error(e) => syntax_error(format!("未知错误 {e}")),
        }
    }
}
