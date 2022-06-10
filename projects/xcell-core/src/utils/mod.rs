use std::{
    fs::File,
    hash::{Hash, Hasher},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use calamine::{open_workbook_auto, DataType, Reader};
use pathdiff::diff_paths;
use twox_hash::XxHash64;

use array2d::Array2D;

use crate::{
    typing::{XCellTyped, XCellValue},
    CalamineTable, Validation, XCellHeader, XCellHeaders, XError, XResult,
};

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
pub fn read_table_headers(table: &CalamineTable) -> XResult<XCellHeaders> {
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
            headers.push(XCellHeader { summary: data.to_string(), column: i, typing, field_name, details: "".to_string() })
        }
    }
    Ok(XCellHeaders::new(headers).check_enumerate())
}

pub fn read_table_data(table: &CalamineTable, typing: &XCellHeaders) -> Validation<Array2D<XCellValue>> {
    let mut errors = vec![];
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
                Err(e) => errors.push(XError { kind: Box::new(e), path: None, position: Some((x, y)) }),
            }
        }
    }
    Validation::Success { value: matrix, diagnostics: vec![] }
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
