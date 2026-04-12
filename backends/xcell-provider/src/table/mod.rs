//! 表格读取模块
//!
//! 提供统一的表格读取接口，支持 CSV、TSV、Excel 等多种格式。

use std::path::Path;

mod file_format;
mod reader;

pub use file_format::{FileFormat, FileFormatDetector};
pub use reader::{convert_data, ArcTableReader, CalamineTable, CsvRows, CsvTable, TableReader, TsvRows, TsvTable, XCellHeader};
pub use xcell_core::{XCellAccess, XDocument};
pub use xcell_parser::FieldConstraint;

use xcell_core::{FieldConfig, TableLineMode, TypeMetaInfo, XError, XErrorKind, XResult};

/// 根据文件路径自动检测文件格式并加载表格
///
/// # Parameters
/// - `path`: 表格文件的路径
///
/// # Returns
/// - 成功时返回实现了 `TableReader` trait 的实例
/// - 失败时返回错误
pub fn load_table(path: &Path) -> XResult<Box<dyn TableReader>> {
    load_table_with_config(path, &TypeMetaInfo::default())
}

/// 根据文件路径和配置自动检测文件格式并加载表格
///
/// # Parameters
/// - `path`: 表格文件的路径
/// - `config`: 类型配置
///
/// # Returns
/// - 成功时返回实现了 `TableReader` trait 的实例
/// - 失败时返回错误
pub fn load_table_with_config(path: &Path, config: &TypeMetaInfo) -> XResult<Box<dyn TableReader>> {
    let format = FileFormatDetector::detect(path)?;

    match format {
        FileFormat::Excel => {
            let table = CalamineTable::load_with_config(path, config)?;
            Ok(Box::new(table))
        }
        FileFormat::Csv => {
            let table = CsvTable::load(path)?;
            Ok(Box::new(table))
        }
        FileFormat::Tsv => {
            let table = TsvTable::load(path)?;
            Ok(Box::new(table))
        }
        FileFormat::Ods => Err(XError::new(XErrorKind::TableError("ODS 格式暂不支持读取".to_string()))),
        FileFormat::Unknown => Err(XError::new(XErrorKind::TableError(format!("无法检测文件格式: {:?}", path)))),
    }
}

/// 根据文件路径和完整配置加载表格
///
/// # Parameters
/// - `path`: 表格文件的路径
/// - `typing`: 类型元信息
/// - `line`: 行模式配置
/// - `fields`: 字段配置列表
///
/// # Returns
/// - 成功时返回实现了 `TableReader` trait 的实例
/// - 失败时返回错误
pub fn load_table_with_full_config(
    path: &Path,
    typing: &TypeMetaInfo,
    line: &TableLineMode,
    fields: &[FieldConfig],
) -> XResult<Box<dyn TableReader>> {
    let format = FileFormatDetector::detect(path)?;

    match format {
        FileFormat::Excel => {
            let table = CalamineTable::load_with_full_config(path, typing, line, fields)?;
            Ok(Box::new(table))
        }
        FileFormat::Csv => {
            let table = CsvTable::load(path)?;
            Ok(Box::new(table))
        }
        FileFormat::Tsv => {
            let table = TsvTable::load(path)?;
            Ok(Box::new(table))
        }
        FileFormat::Ods => Err(XError::new(XErrorKind::TableError("ODS 格式暂不支持读取".to_string()))),
        FileFormat::Unknown => Err(XError::new(XErrorKind::TableError(format!("无法检测文件格式: {:?}", path)))),
    }
}

/// 读取 Excel 文件里的第一张表
///
/// # Parameters
/// - `path`: 表格文件的路径
///
/// # Returns
/// - 成功时返回 Excel 表格的范围
/// - 失败时返回错误
pub fn find_first_table(path: &Path) -> XResult<calamine::Range<calamine::Data>> {
    use calamine::Reader;
    let mut workbook = calamine::open_workbook_auto(path).map_err(|e| XError::new(XErrorKind::IOError(format!("{:?}", e))))?;
    let ranges = match workbook.worksheet_range_at(0) {
        None => return Err(XError::new(XErrorKind::TableError("找不到配置表, 文件是空的, 或者表格式非法".to_string()))),
        Some(s) => s.map_err(|e| XError::new(XErrorKind::IOError(format!("{:?}", e)))),
    }?;
    Ok(ranges)
}
