use chrono::{NaiveDateTime, TimeZone};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeDescription {
    /// `new DateTime(1, 1, 1, 0, 0, 0, DateTimeKind.Utc)`
    pub default: DateTime,
}

impl Default for TimeDescription {
    fn default() -> Self {
        TimeDescription { default: DateTime::from(SystemTime::now()) }
    }
}

impl TimeDescription {
    pub fn parse_cell(&self, cell: &DataType) -> Result<DateTime, XErrorKind> {
        match cell {
            DataType::DateTime(time) => {
                let ntv = NaiveDateTime::from_timestamp(*time as i64, 0);
                let utc = Utc.from_utc_datetime(&ntv);
                Ok(utc)
            }
            DataType::String(s) => match DateTime::from_str(s) {
                Ok(o) => Ok(o),
                Err(_) => syntax_error(format!("{} 无法解析为 time 类型", s)),
            },
            DataType::Empty => Ok(self.default.clone()),
            _ => syntax_error(format!("{} 无法解析为 time 类型", cell.to_string())),
        }
    }
}
