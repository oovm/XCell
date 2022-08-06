use std::{
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

mod watcher;

pub use self::workspace::*;
use crate::{CalamineTable, XCellHeader};
use array2d::Array2D;
use calamine::{open_workbook_auto, DataType, Reader};
use itertools::Itertools;
use pathdiff::diff_paths;
use twox_hash::XxHash64;
use xcell_errors::{XError, XResult};
use xcell_types::{ExtraTypes, XCellTyped, XCellValue};
mod workspace;

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

pub fn read_table_data(table: &CalamineTable, typing: &XCellHeaders, path: &Path) -> XResult<Array2D<XCellValue>> {
    let rows = table.rows().skip(3).filter(|v| first_not_nil(v));
    let row_count = table.rows().skip(3).filter(|v| first_not_nil(v)).count();
    let col_count = typing.len();
    let mut matrix = Array2D::filled_with(XCellValue::Boolean(false), row_count, col_count);
    for (x, row_raw) in rows.enumerate() {
        for (y, typed) in typing.iter().enumerate() {
            match typed.parse_cell(row_raw) {
                Ok(o) => {
                    matrix.set(x, y, o).ok();
                }
                Err(e) => {
                    log::error!("{}", e.with_path(path).with_xy(x, y))
                }
            }
        }
    }
    Ok(matrix)
}

/// 确保第一行的 id 不是空的
///
/// 如果是空的, 那么就认为数据非法
pub fn first_not_nil(row: &[DataType]) -> bool {
    match row.first() {
        Some(s) => match s {
            DataType::Int(_) => true,
            DataType::Float(_) => true,
            DataType::String(s) => !s.is_empty(),
            DataType::Bool(_) => true,
            DataType::DateTime(_) => true,
            DataType::Error(_) => false,
            DataType::Empty => false,
        },
        None => false,
    }
}

pub fn xx_hash<T>(body: T) -> u64
where
    T: Hash,
{
    let mut hasher = XxHash64::default();
    body.hash(&mut hasher);
    hasher.finish()
}

pub fn xx_file(path: &Path) -> XResult<u64> {
    let mut hasher = XxHash64::default();
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }
    Ok(hasher.finish())
}

pub fn split_file_name(s: &str) -> String {
    let mut all = vec![];
    for name in s.split(|c| c == '/' || c == '\\') {
        if !name.trim().is_empty() {
            all.push(name)
        }
    }
    all.join("/")
}

pub fn split_namespace(s: &str) -> Vec<&str> {
    let mut all = vec![];
    for s in s.split("::") {
        for name in s.split('.') {
            if !name.trim().is_empty() {
                all.push(name)
            }
        }
    }
    all
}
