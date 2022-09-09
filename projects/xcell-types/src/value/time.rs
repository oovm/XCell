use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TimeDescription {
    /// `new DateTime(1, 1, 1, 0, 0, 0, DateTimeKind.Utc)`
    pub default: Option<DateTime>,
}

impl TimeDescription {
    pub fn with_default(mut self, time: &str) -> Self {
        if let Ok(o) = DateTime::from_str(time) {
            self.default = Some(o)
        }
        self
    }
}

impl TimeDescription {
    pub fn parse_cell(&self, cell: &DataType) -> XResult<XCellValueKind> {
        let _ = self.parse_value(cell)?;
        todo!()
    }

    fn parse_value(&self, cell: &DataType) -> XResult<DateTime> {
        match cell {
            DataType::DateTime(time) => {
                let ntv = NaiveDateTime::from_timestamp_opt(*time as i64, 0).unwrap_or_default();
                let utc = Utc.from_utc_datetime(&ntv);
                Ok(utc)
            }
            DataType::String(s) => match DateTime::from_str(s) {
                Ok(o) => Ok(o),
                Err(_) => syntax_error(format!("{} 无法解析为 time 类型", s)),
            },
            DataType::Empty => match &self.default {
                Some(s) => Ok(*s),
                None => Ok(DateTime::default()),
            },
            _ => syntax_error(format!("{} 无法解析为 time 类型", cell)),
        }
    }
}
