//! 文件格式检测模块
//!
//! 提供文件格式的自动检测功能。

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::{XError, XErrorKind, XResult};

/// 文件格式枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileFormat {
    /// Excel 格式（.xlsx, .xls）
    Excel,
    /// CSV 格式（.csv）
    Csv,
    /// TSV 格式（.tsv）
    Tsv,
    /// ODS 格式（.ods）
    Ods,
    /// 未知格式
    Unknown,
}

/// 文件格式检测器
pub struct FileFormatDetector;

impl FileFormatDetector {
    /// 根据文件路径和内容检测文件格式
    ///
    /// 首先根据文件扩展名判断，如果扩展名无法判断，则根据文件内容判断。
    ///
    /// # Parameters
    /// - `path`: 文件路径
    ///
    /// # Returns
    /// - 检测到的文件格式
    pub fn detect(path: &Path) -> XResult<FileFormat> {
        if !path.exists() {
            return Err(XError::new(XErrorKind::IOError(format!("文件不存在: {:?}", path))));
        }

        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap_or("");
            match ext.to_lowercase().as_str() {
                "xlsx" | "xls" => return Ok(FileFormat::Excel),
                "csv" => return Ok(FileFormat::Csv),
                "tsv" => return Ok(FileFormat::Tsv),
                "ods" => return Ok(FileFormat::Ods),
                _ => (),
            }
        }

        Self::detect_by_content(path)
    }

    /// 根据文件内容检测文件格式
    ///
    /// 通过检查 Excel 魔术数字和统计逗号/制表符数量来判断文件格式。
    ///
    /// # Parameters
    /// - `path`: 文件路径
    ///
    /// # Returns
    /// - 检测到的文件格式
    fn detect_by_content(path: &Path) -> XResult<FileFormat> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; 8192];
        let bytes_read = reader.read(&mut buffer)?;
        buffer.truncate(bytes_read);

        if buffer.len() >= 4 {
            let magic = &buffer[0..4];
            if magic == &[0x50, 0x4B, 0x03, 0x04] {
                return Ok(FileFormat::Excel);
            }
        }

        if !buffer.is_empty() {
            let content = String::from_utf8_lossy(&buffer);

            let comma_count = content.matches(',').count();
            let tab_count = content.matches('\t').count();

            if comma_count > tab_count {
                return Ok(FileFormat::Csv);
            }
            else if tab_count > comma_count {
                return Ok(FileFormat::Tsv);
            }
        }

        Ok(FileFormat::Unknown)
    }

    /// 根据文件格式获取对应的读取器类型
    ///
    /// # Parameters
    /// - `format`: 文件格式
    ///
    /// # Returns
    /// - 读取器类型名称
    pub fn get_reader_type(format: &FileFormat) -> &'static str {
        match format {
            FileFormat::Excel => "excel",
            FileFormat::Csv => "csv",
            FileFormat::Tsv => "tsv",
            FileFormat::Ods => "ods",
            FileFormat::Unknown => "unknown",
        }
    }
}
