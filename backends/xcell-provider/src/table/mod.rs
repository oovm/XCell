//! 表格读取模块
//!
//! 提供统一的表格读取接口，支持 CSV、TSV、Excel 等多种格式。

use std::path::Path;

mod file_format;
mod reader;

pub use file_format::{FileFormat, FileFormatDetector};
pub use reader::{CsvTable, ExcelTable, TableReader, TsvTable, XCellAccess, XCellHeader, XDocument};

use xcell_core::{TypeMetaInfo, XError, XErrorKind, XResult};

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
    let format = FileFormatDetector::detect(path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;

    match format {
        FileFormat::Excel => {
            let table = ExcelTable::load(path)?;
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
        FileFormat::Unknown => Err(XError::new(XErrorKind::TableError(format!("无法检测文件格式: {:?}", path)))),
    }
}
