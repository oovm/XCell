use std::path::{Path, PathBuf};

use calamine::{open_workbook_auto, DataType, Reader};
use pathdiff::diff_paths;

use crate::{typing::XCellTyped, CalamineTable, XCellHeader, XError, XResult};

/// 读取 Excel 文件里的第一张表
///
/// # Arguments
///
/// * `path`:
///
/// returns: Result<Range<DataType>, XError>
///
/// # Examples
///
/// ```
/// use xcell_core;
/// ```
pub fn find_first_table(path: &PathBuf) -> XResult<CalamineTable> {
    let mut workbook = open_workbook_auto(path)?;
    let ranges = match workbook.worksheet_range_at(0) {
        None => return Err(XError::table_error("找不到配置表, 文件是空的, 或者表格式非法")),
        Some(s) => s?,
    };
    Ok(ranges)
}

/// 获取表格的前三行
///
/// # Arguments
///
/// * `table`:
///
/// returns: Result<Vec<XCellHeader, Global>, XError>
///
/// # Examples
///
///
/// ```
/// use xcell_core;
/// ```
pub fn read_table_headers(table: &CalamineTable) -> XResult<Vec<XCellHeader>> {
    let mut headers = vec![];
    let row = match table.rows().nth(0) {
        Some(s) => s,
        None => return Err(XError::table_error("找不到描述, 表第一行格式非法")),
    };
    for (i, data) in row.iter().enumerate() {
        if !data.is_empty() {
            let typing = match table.get_value((1, i as u32)) {
                Some(s) => XCellTyped::from(s),
                None => continue,
            };
            let field_name = match table.get_value((2, i as u32)) {
                Some(s) => s.to_string(),
                None => continue,
            };
            headers.push(XCellHeader { comment: data.to_string(), column: i, typing, field_name, details: "".to_string() })
        }
    }
    Ok(headers)
}

pub fn read_table_data(table: &CalamineTable) {}

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

/// 取得相对路径
///
/// # Arguments
///
/// * `this`:
/// * `root`:
///
/// # Examples
///
/// ```
/// use xcell_core;
/// ```
pub fn make_relative<A, B>(this: A, root: B) -> XResult<PathBuf>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = this.as_ref().canonicalize()?;
    let base = root.as_ref().canonicalize()?;
    match diff_paths(&path, &base) {
        Some(o) => Ok(o),
        None => Err(XError::table_error(format!("无法取得相对路径 {path:?} {base:?}"))),
    }
}
