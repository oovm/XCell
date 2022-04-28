use std::path::Path;

use calamine::{open_workbook_auto, Reader};

use crate::{XError, XResult};

type CalamineTable = calamine::Range<calamine::DataType>;

pub fn find_first_table(path: &Path) -> XResult<CalamineTable> {
    let mut workbook = open_workbook_auto(path)?;
    let ranges = match workbook.worksheet_range_at(0) {
        None => return Err(XError::table_error("找不到配置表, 文件是空的, 或者表格式非法")),
        Some(s) => s?,
    };
    Ok(ranges)
}

pub fn find_first_row(table: &CalamineTable) -> XResult<()> {
    let tables = match table.rows().nth(0) {
        Some(s) => s,
        None => return Err(XError::table_error("找不到第一行, 表格式非法")),
    };
    for (i, data) in tables.iter().enumerate() {
        println!("{} {:#?}", i, data)
    }
    Ok(())
}
