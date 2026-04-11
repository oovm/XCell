use std::fmt::{Display, Formatter};

/// 表示电子表格单元格的数据值
#[derive(Debug, Clone)]
pub enum Data {
    /// 整数值
    Int(i64),
    /// 浮点数值
    Float(f64),
    /// 字符串值
    String(String),
    /// 布尔值
    Bool(bool),
    /// Excel 序列号日期时间值
    DateTime(f64),
    /// ISO 8601 日期时间字符串
    DateTimeIso(String),
    /// ISO 8601 持续时间字符串
    DurationIso(String),
    /// 错误值
    Error(String),
    /// 空单元格
    Empty,
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Int(v) => write!(f, "{}", v),
            Data::Float(v) => write!(f, "{}", v),
            Data::String(v) => write!(f, "{}", v),
            Data::Bool(v) => write!(f, "{}", v),
            Data::DateTime(v) => write!(f, "{}", v),
            Data::DateTimeIso(v) => write!(f, "{}", v),
            Data::DurationIso(v) => write!(f, "{}", v),
            Data::Error(v) => write!(f, "{}", v),
            Data::Empty => write!(f, ""),
        }
    }
}

/// 将 Excel 序列号日期时间值转换为 chrono NaiveDateTime
///
/// Excel 以 1899-12-30 为纪元起点，并包含 Lotus 1-2-3 的闰年错误
/// （错误地将 1900-02-29 视为有效日期）
#[cfg(feature = "chrono")]
pub fn excel_serial_to_naive_datetime(serial: f64) -> Option<chrono::NaiveDateTime> {
    use chrono::{Duration, NaiveDate, NaiveDateTime};
    let base = NaiveDate::from_ymd_opt(1899, 12, 30)?.and_hms_opt(0, 0, 0)?;
    let days = serial.floor() as i64;
    let fraction = serial - serial.floor();
    let secs = (fraction * 86400.0).round() as i64;
    base.checked_add_signed(Duration::days(days))?
        .checked_add_signed(Duration::seconds(secs))
}
