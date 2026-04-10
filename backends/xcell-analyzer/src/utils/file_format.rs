use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::XResult;

/// 文件格式枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileFormat {
    /// Excel 文件格式 (xlsx, xls)
    Excel,
    /// CSV 文件格式
    Csv,
    /// TSV 文件格式
    Tsv,
    /// 未知文件格式
    Unknown,
}

/// 文件格式检测器
pub struct FileFormatDetector;

impl FileFormatDetector {
    /// 根据文件路径和内容检测文件格式
    ///
    /// # Parameters
    /// - `path`: 文件路径
    ///
    /// # Returns
    /// - 检测到的文件格式
    pub fn detect(path: &Path) -> XResult<FileFormat> {
        // 首先根据文件扩展名判断
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            match ext_str.as_str() {
                "xlsx" | "xls" => return Ok(FileFormat::Excel),
                "csv" => return Ok(FileFormat::Csv),
                "tsv" => return Ok(FileFormat::Tsv),
                _ => {}
            }
        }

        // 如果扩展名无法判断，尝试根据文件内容判断
        Self::detect_by_content(path)
    }

    /// 根据文件内容检测文件格式
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

        // 检查 Excel 文件的魔术数字
        if buffer.len() >= 4 {
            let magic = &buffer[0..4];
            // Excel 2007+ (xlsx) 的魔术数字
            if magic == &[0x50, 0x4B, 0x03, 0x04] {
                return Ok(FileFormat::Excel);
            }
        }

        // 检查 CSV/TSV 文件
        if !buffer.is_empty() {
            let content = String::from_utf8_lossy(&buffer);

            // 统计逗号和制表符的数量
            let comma_count = content.matches(',').count();
            let tab_count = content.matches('\t').count();

            if comma_count > tab_count {
                return Ok(FileFormat::Csv);
            }
            else if tab_count > comma_count {
                return Ok(FileFormat::Tsv);
            }
        }

        // 无法检测到文件格式
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
            FileFormat::Unknown => "unknown",
        }
    }
}
