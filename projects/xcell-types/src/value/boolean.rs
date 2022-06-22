use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BooleanDescription {
    pub accept: Vec<String>,
    pub reject: Vec<String>,
    pub default: bool,
}

impl BooleanDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValue> {
        self.parse_value(cell).map(XCellValue::Boolean)
    }
    fn parse_value(&self, cell: &DataType) -> XResult<bool> {
        match cell {
            DataType::String(s) => {
                if self.accept.contains(s) {
                    Ok(true)
                }
                else if self.reject.contains(s) {
                    Ok(false)
                }
                else {
                    syntax_error(format!("{} 无法解析为 bool 类型", s))
                }
            }
            DataType::Bool(v) => Ok(*v),
            DataType::Empty => Ok(self.default),
            DataType::Error(e) => syntax_error(format!("未知错误 {e}")),
            _ => type_mismatch("Boolean", cell),
        }
    }
}
