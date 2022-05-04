use std::path::PathBuf;

use calamine::{open_workbook_auto, DataType, Reader};

use crate::{CalamineTable, XError, XResult};

pub fn find_first_table(path: PathBuf) -> XResult<CalamineTable> {
    let mut workbook = open_workbook_auto(&path)?;
    let ranges = match workbook.worksheet_range_at(0) {
        None => return Err(XError::table_error("找不到配置表, 文件是空的, 或者表格式非法", &path)),
        Some(s) => s?,
    };
    Ok(ranges)
}

pub fn first_is_nil(row: &[DataType]) -> bool {
    match row.first() {
        Some(s) => match s {
            DataType::Int(_) => false,
            DataType::Float(_) => false,
            DataType::String(s) => s.is_empty(),
            DataType::Bool(_) => false,
            DataType::DateTime(_) => false,
            DataType::Error(_) => true,
            DataType::Empty => true,
        },
        None => true,
    }
}
