//! 文件格式检测模块
//!
//! 提供文件格式的自动检测功能。

use std::path::Path;

/// 文件格式枚举
#[derive(Debug, PartialEq, Eq)]
pub enum FileFormat {
    /// Excel 格式（.xlsx, .xls）
    Excel,
    /// CSV 格式（.csv）
    Csv,
    /// TSV 格式（.tsv）
    Tsv,
    /// 未知格式
    Unknown,
}

/// 文件格式检测器
pub struct FileFormatDetector;

impl FileFormatDetector {
    /// 根据文件路径检测文件格式
    ///
    /// # Parameters
    /// - `path`: 文件路径
    ///
    /// # Returns
    /// - 检测到的文件格式
    pub fn detect(path: &Path) -> Result<FileFormat, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("文件不存在: {:?}", path))));
        }

        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap_or("");
            match ext.to_lowercase().as_str() {
                "xlsx" | "xls" => return Ok(FileFormat::Excel),
                "csv" => return Ok(FileFormat::Csv),
                "tsv" => return Ok(FileFormat::Tsv),
                _ => (),
            }
        }

        Ok(FileFormat::Unknown)
    }
}
