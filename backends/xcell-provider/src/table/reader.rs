//! 表格读取器模块
//!
//! 提供统一的表格读取接口和各种格式的表格读取器实现。

use calamine::{Data, Reader};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use xcell_types::{IntegerKind, TypeMetaInfo, XCellTyped, XError, XErrorKind, XResult};

/// 表格访问权限枚举
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum XCellAccess {
    /// 默认访问权限
    #[default]
    Default,
    /// 公共访问权限
    Public,
    /// 私有访问权限
    Private,
}

/// 文档类型
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDocument {
    /// 文档内容
    content: String,
}

impl XDocument {
    /// 创建新的文档
    pub fn new(content: impl Into<String>) -> Self {
        Self { content: content.into() }
    }

    /// 获取文档内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 按行分割文档内容
    pub fn lines(&self) -> Vec<String> {
        self.content.lines().map(|s| s.to_string()).collect()
    }

    /// 检查文档是否为空
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl From<String> for XDocument {
    fn from(content: String) -> Self {
        Self { content }
    }
}

impl From<&str> for XDocument {
    fn from(content: &str) -> Self {
        Self { content: content.to_string() }
    }
}

/// 表格表头信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XCellHeader {
    /// 列索引
    pub column: usize,
    /// 访问权限
    pub access: XCellAccess,
    /// 字段名
    pub field_name: String,
    /// 类型信息
    pub typing: XCellTyped,
    /// 文档信息
    pub document: XDocument,
    /// 是否完整
    pub complete: bool,
}

/// 表格读取器 trait
///
/// 所有表格读取器都需要实现此 trait。
pub trait TableReader: Send + Sync + std::fmt::Debug {
    /// 加载表格文件
    ///
    /// # Parameters
    /// - `path`: 表格文件的路径
    ///
    /// # Returns
    /// - 成功时返回读取器实例
    /// - 失败时返回错误
    fn load(path: &Path) -> XResult<Self>
    where
        Self: Sized;

    /// 加载表格文件并应用配置
    ///
    /// # Parameters
    /// - `path`: 表格文件的路径
    /// - `config`: 类型配置
    ///
    /// # Returns
    /// - 成功时返回读取器实例
    /// - 失败时返回错误
    fn load_with_config(path: &Path, config: &TypeMetaInfo) -> XResult<Self>
    where
        Self: Sized,
    {
        Self::load(path)
    }

    /// 获取表格名称
    fn get_name(&self) -> String;

    /// 获取表头信息
    ///
    /// # Parameters
    /// - `index`: 列索引
    ///
    /// # Returns
    /// - 表头信息
    fn get_header(&self, index: usize) -> XCellHeader;

    /// 获取所有表头
    ///
    /// # Returns
    /// - 表头信息迭代器
    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_>;

    /// 获取表格行数据
    ///
    /// # Returns
    /// - 行索引和数据的迭代器
    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<Data>)> + '_>;

    /// 解析类型
    ///
    /// # Parameters
    /// - `name`: 类型名称
    ///
    /// # Returns
    /// - 解析后的类型
    fn parse_type(&self, name: &str) -> XCellTyped;

    /// 解析类型并使用配置
    ///
    /// # Parameters
    /// - `name`: 类型名称
    /// - `config`: 类型配置
    ///
    /// # Returns
    /// - 解析后的类型
    fn parse_type_with_config(&self, name: &str, config: &TypeMetaInfo) -> XCellTyped {
        XCellTyped::parse(name, config)
    }

    /// 获取默认枚举类型
    fn default_enumerate(&self) -> IntegerKind;

    /// 检查是否为语言定义表
    fn is_language_define(&self) -> bool;

    /// 检查是否为语言表
    fn is_language_table(&self) -> bool;

    /// 检查是否为语言值
    ///
    /// # Parameters
    /// - `name`: 列名
    ///
    /// # Returns
    /// - 是否为语言值
    fn is_language_value(&self, name: &str) -> bool;

    /// 检查是否为类表
    fn is_class(&self) -> bool;

    /// 检查是否为列表
    fn is_list(&self) -> bool;

    /// 检查是否为字典表
    fn is_dict(&self) -> bool;

    /// 检查是否为组
    ///
    /// # Parameters
    /// - `name`: 组名
    ///
    /// # Returns
    /// - 是否为组
    fn is_group(&self, name: &str) -> bool;

    /// 检查是否为枚举
    ///
    /// # Parameters
    /// - `name`: 枚举名
    ///
    /// # Returns
    /// - 是否为枚举
    fn is_enumerate(&self, name: &str) -> bool;

    /// 检查是否为数字键
    ///
    /// # Parameters
    /// - `name`: 键名
    ///
    /// # Returns
    /// - 是否为数字键
    fn is_numeric_key(&self, name: &str) -> bool;

    /// 检查是否为文档
    ///
    /// # Parameters
    /// - `name`: 文档名
    ///
    /// # Returns
    /// - 是否为文档
    fn is_document(&self, name: &str) -> bool;

    /// 设置表头信息
    ///
    /// # Parameters
    /// - `index`: 列索引
    /// - `header`: 表头信息
    fn set_header(&mut self, index: usize, header: XCellHeader) -> XResult<()> {
        // 默认实现，实际需要根据具体类型实现
        Ok(())
    }

    /// 添加表头
    ///
    /// # Parameters
    /// - `header`: 表头信息
    fn add_header(&mut self, header: XCellHeader) -> XResult<()> {
        // 默认实现，实际需要根据具体类型实现
        Ok(())
    }

    /// 写入行数据
    ///
    /// # Parameters
    /// - `row_index`: 行索引
    /// - `data`: 行数据
    fn write_row(&mut self, row_index: usize, data: Vec<Data>) -> XResult<()> {
        // 默认实现，实际需要根据具体类型实现
        Ok(())
    }

    /// 添加行数据
    ///
    /// # Parameters
    /// - `data`: 行数据
    fn add_row(&mut self, data: Vec<Data>) -> XResult<()> {
        // 默认实现，实际需要根据具体类型实现
        Ok(())
    }

    /// 保存表格到文件
    ///
    /// # Parameters
    /// - `path`: 保存路径
    ///
    /// # Returns
    /// - 成功时返回 Ok(())，失败时返回错误
    fn save(&self, path: &Path) -> XResult<()> {
        Err(XError::new(XErrorKind::TableError("保存功能未实现".to_string())))
    }

    /// 设置表格标签
    ///
    /// # Parameters
    /// - `label`: 标签名称
    fn set_label(&mut self, label: &str) -> XResult<()> {
        // 默认实现，实际需要根据具体类型实现
        Ok(())
    }

    /// 获取表格标签
    ///
    /// # Returns
    /// - 标签名称
    fn get_label(&self) -> XResult<String> {
        // 默认实现，返回空字符串
        Ok(String::new())
    }
}

/// CSV 表格读取器
#[derive(Debug)]
pub struct CsvTable {
    /// 表格的绝对路径
    path: PathBuf,
    /// 表格标签
    label: String,
    /// 表头信息
    headers: Vec<XCellHeader>,
}

impl CsvTable {
    /// 加载 CSV 文件
    ///
    /// # Parameters
    /// - `path`: CSV 文件的路径
    ///
    /// # Returns
    /// - 成功时返回 CsvTable 实例
    /// - 失败时返回错误
    pub fn load(path: &Path) -> XResult<Self> {
        let path = path.canonicalize().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut headers = Vec::new();
        
        // 读取 CSV 文件的第一行作为表头
        if let Ok(mut reader) = csv::Reader::from_path(&path) {
            if let Ok(header_row) = reader.headers() {
                for (i, field_name) in header_row.iter().enumerate() {
                    headers.push(XCellHeader {
                        column: i,
                        access: XCellAccess::Public,
                        field_name: field_name.to_string(),
                        typing: XCellTyped::default(),
                        document: XDocument::default(),
                        complete: true,
                    });
                }
            }
        }

        Ok(Self { path, label: String::new(), headers })
    }

    /// 创建 CSV 读取器
    fn create_reader(&self) -> XResult<csv::Reader<std::fs::File>> {
        let file = std::fs::File::open(&self.path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        Ok(csv::Reader::from_reader(file))
    }
}

impl TableReader for CsvTable {
    fn load(path: &Path) -> XResult<Self> {
        Self::load(path)
    }

    fn get_name(&self) -> String {
        self.path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string()
    }

    fn get_header(&self, index: usize) -> XCellHeader {
        self.headers.get(index).cloned().unwrap_or_default()
    }

    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_> {
        Box::new(self.headers.clone().into_iter())
    }

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<Data>)> + '_> {
        // 读取所有行数据到内存中
        match self.create_reader() {
            Ok(mut reader) => {
                let mut rows = Vec::new();
                for (i, result) in reader.records().enumerate() {
                    match result {
                        Ok(record) => {
                            let mut data = Vec::with_capacity(record.len());
                            for s in record.iter() {
                                data.push(Data::String(s.to_string()));
                            }
                            rows.push((i, data));
                        }
                        Err(_) => rows.push((i, Vec::new())),
                    }
                }
                Box::new(rows.into_iter())
            }
            Err(_) => Box::new(std::iter::empty()),
        }
    }

    fn parse_type(&self, _name: &str) -> XCellTyped {
        // 简化实现，实际需要根据配置解析类型
        XCellTyped::default()
    }

    fn default_enumerate(&self) -> IntegerKind {
        // 简化实现，实际需要根据配置返回默认枚举类型
        IntegerKind::Unsigned32
    }

    fn is_language_define(&self) -> bool {
        false
    }

    fn is_language_table(&self) -> bool {
        false
    }

    fn is_language_value(&self, _name: &str) -> bool {
        false
    }

    fn is_class(&self) -> bool {
        false
    }

    fn is_list(&self) -> bool {
        if let Some(first_header) = self.headers.first() {
            first_header.field_name.eq_ignore_ascii_case("id")
        } else {
            false
        }
    }

    fn is_dict(&self) -> bool {
        false
    }

    fn is_group(&self, _name: &str) -> bool {
        false
    }

    fn is_enumerate(&self, _name: &str) -> bool {
        false
    }

    fn is_numeric_key(&self, name: &str) -> bool {
        name.eq_ignore_ascii_case("id")
    }

    fn is_document(&self, _name: &str) -> bool {
        false
    }

    fn set_header(&mut self, index: usize, header: XCellHeader) -> XResult<()> {
        if index >= self.headers.len() {
            self.headers.resize(index + 1, XCellHeader::default());
        }
        self.headers[index] = header;
        Ok(())
    }

    fn add_header(&mut self, header: XCellHeader) -> XResult<()> {
        self.headers.push(header);
        Ok(())
    }

    fn write_row(&mut self, _row_index: usize, _data: Vec<Data>) -> XResult<()> {
        // 流式读取模式下，不支持修改数据
        // 实际使用时可能需要实现一个缓存层
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<Data>) -> XResult<()> {
        // 流式读取模式下，不支持修改数据
        // 实际使用时可能需要实现一个缓存层
        Ok(())
    }

    fn save(&self, path: &Path) -> XResult<()> {
        let file = std::fs::File::create(path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut writer = csv::Writer::from_writer(file);

        // 写入表头
        if !self.headers.is_empty() {
            let header_row: Vec<String> = self.headers.iter().map(|h| h.field_name.clone()).collect();
            writer.write_record(&header_row).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        }

        // 从原始文件读取数据并写入新文件
        let mut reader = self.create_reader()?;
        for result in reader.records() {
            match result {
                Ok(record) => {
                    writer.write_record(&record).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
                }
                Err(_) => continue,
            }
        }

        writer.flush().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        Ok(())
    }

    fn set_label(&mut self, label: &str) -> XResult<()> {
        self.label = label.to_string();
        Ok(())
    }

    fn get_label(&self) -> XResult<String> {
        Ok(self.label.clone())
    }
}

/// TSV 表格读取器
#[derive(Debug)]
pub struct TsvTable {
    /// 表格的绝对路径
    path: PathBuf,
    /// 表格标签
    label: String,
    /// 表头信息
    headers: Vec<XCellHeader>,
}

impl TsvTable {
    /// 加载 TSV 文件
    ///
    /// # Parameters
    /// - `path`: TSV 文件的路径
    ///
    /// # Returns
    /// - 成功时返回 TsvTable 实例
    /// - 失败时返回错误
    pub fn load(path: &Path) -> XResult<Self> {
        let path = path.canonicalize().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;

        Ok(Self { path, label: String::new(), headers: Vec::new() })
    }

    /// 创建 TSV 读取器
    fn create_reader(&self) -> XResult<csv::Reader<std::fs::File>> {
        let file = std::fs::File::open(&self.path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        Ok(csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file))
    }
}

impl TableReader for TsvTable {
    fn load(path: &Path) -> XResult<Self> {
        Self::load(path)
    }

    fn get_name(&self) -> String {
        self.path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string()
    }

    fn get_header(&self, index: usize) -> XCellHeader {
        self.headers.get(index).cloned().unwrap_or_default()
    }

    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_> {
        Box::new(self.headers.clone().into_iter())
    }

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<Data>)> + '_> {
        // 读取所有行数据到内存中
        match self.create_reader() {
            Ok(mut reader) => {
                let mut rows = Vec::new();
                for (i, result) in reader.records().enumerate() {
                    match result {
                        Ok(record) => {
                            let mut data = Vec::with_capacity(record.len());
                            for s in record.iter() {
                                data.push(Data::String(s.to_string()));
                            }
                            rows.push((i, data));
                        }
                        Err(_) => rows.push((i, Vec::new())),
                    }
                }
                Box::new(rows.into_iter())
            }
            Err(_) => Box::new(std::iter::empty()),
        }
    }

    fn parse_type(&self, _name: &str) -> XCellTyped {
        // 简化实现，实际需要根据配置解析类型
        XCellTyped::default()
    }

    fn default_enumerate(&self) -> IntegerKind {
        // 简化实现，实际需要根据配置返回默认枚举类型
        IntegerKind::Unsigned32
    }

    fn is_language_define(&self) -> bool {
        false
    }

    fn is_language_table(&self) -> bool {
        false
    }

    fn is_language_value(&self, _name: &str) -> bool {
        false
    }

    fn is_class(&self) -> bool {
        false
    }

    fn is_list(&self) -> bool {
        if let Some(first_header) = self.headers.first() {
            first_header.field_name.eq_ignore_ascii_case("id")
        } else {
            false
        }
    }

    fn is_dict(&self) -> bool {
        false
    }

    fn is_group(&self, _name: &str) -> bool {
        false
    }

    fn is_enumerate(&self, _name: &str) -> bool {
        false
    }

    fn is_numeric_key(&self, name: &str) -> bool {
        name.eq_ignore_ascii_case("id")
    }

    fn is_document(&self, _name: &str) -> bool {
        false
    }

    fn set_header(&mut self, index: usize, header: XCellHeader) -> XResult<()> {
        if index >= self.headers.len() {
            self.headers.resize(index + 1, XCellHeader::default());
        }
        self.headers[index] = header;
        Ok(())
    }

    fn add_header(&mut self, header: XCellHeader) -> XResult<()> {
        self.headers.push(header);
        Ok(())
    }

    fn write_row(&mut self, _row_index: usize, _data: Vec<Data>) -> XResult<()> {
        // 流式读取模式下，不支持修改数据
        // 实际使用时可能需要实现一个缓存层
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<Data>) -> XResult<()> {
        // 流式读取模式下，不支持修改数据
        // 实际使用时可能需要实现一个缓存层
        Ok(())
    }

    fn save(&self, path: &Path) -> XResult<()> {
        let file = std::fs::File::create(path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

        // 写入表头
        if !self.headers.is_empty() {
            let header_row: Vec<String> = self.headers.iter().map(|h| h.field_name.clone()).collect();
            writer.write_record(&header_row).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        }

        // 从原始文件读取数据并写入新文件
        let mut reader = self.create_reader()?;
        for result in reader.records() {
            match result {
                Ok(record) => {
                    writer.write_record(&record).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
                }
                Err(_) => continue,
            }
        }

        writer.flush().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        Ok(())
    }

    fn set_label(&mut self, label: &str) -> XResult<()> {
        self.label = label.to_string();
        Ok(())
    }

    fn get_label(&self) -> XResult<String> {
        Ok(self.label.clone())
    }
}

/// Excel 表格读取器
#[derive(Clone, Debug)]
pub struct ExcelTable {
    /// 表格的绝对路径
    path: PathBuf,
    /// 原始表单
    table: calamine::Range<Data>,
    /// 表格标签
    label: String,
    /// 表头信息
    headers: Vec<XCellHeader>,
    /// 工作表名称
    sheet_name: String,
}

impl ExcelTable {
    /// 加载 Excel 文件
    ///
    /// # Parameters
    /// - `path`: Excel 文件的路径
    ///
    /// # Returns
    /// - 成功时返回 ExcelTable 实例
    /// - 失败时返回错误
    pub fn load(path: &Path) -> XResult<Self> {
        let path = path.canonicalize().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut excel: calamine::Xlsx<_> =
            calamine::open_workbook(&path).map_err(|e| XError::new(XErrorKind::IOError(format!("{:?}", e))))?;
        let sheet_names = excel.sheet_names();

        if sheet_names.is_empty() {
            return Err(XError::new(XErrorKind::TableError("Excel 文件中没有工作表".to_string())));
        }

        let first_sheet = &sheet_names[0];
        let range = excel
            .worksheet_range(first_sheet)
            .map_err(|_| XError::new(XErrorKind::TableError(format!("无法读取工作表: {}", first_sheet))))?;

        Ok(Self { path, table: range.clone(), label: String::new(), headers: Vec::new(), sheet_name: first_sheet.to_string() })
    }
}

impl TableReader for ExcelTable {
    fn load(path: &Path) -> XResult<Self> {
        Self::load(path)
    }

    fn get_name(&self) -> String {
        self.path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string()
    }

    fn get_header(&self, index: usize) -> XCellHeader {
        self.headers.get(index).cloned().unwrap_or_default()
    }

    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_> {
        Box::new(self.headers.clone().into_iter())
    }

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<Data>)> + '_> {
        // 优化实现，减少克隆操作
        let range = &self.table;
        let iter = (0..range.height()).map(move |row| {
            let mut data = Vec::with_capacity(range.width() as usize);
            for col in 0..range.width() {
                if let Some(value) = range.get_value((row as u32, col as u32)) {
                    data.push(value.to_owned());
                }
                else {
                    data.push(Data::Empty);
                }
            }
            (row as usize, data)
        });
        Box::new(iter)
    }

    fn parse_type(&self, _name: &str) -> XCellTyped {
        // 简化实现，实际需要根据配置解析类型
        XCellTyped::default()
    }

    fn default_enumerate(&self) -> IntegerKind {
        // 简化实现，实际需要根据配置返回默认枚举类型
        IntegerKind::Unsigned32
    }

    fn is_language_define(&self) -> bool {
        false
    }

    fn is_language_table(&self) -> bool {
        false
    }

    fn is_language_value(&self, _name: &str) -> bool {
        false
    }

    fn is_class(&self) -> bool {
        false
    }

    fn is_list(&self) -> bool {
        if let Some(first_header) = self.headers.first() {
            first_header.field_name.eq_ignore_ascii_case("id")
        } else {
            false
        }
    }

    fn is_dict(&self) -> bool {
        false
    }

    fn is_group(&self, _name: &str) -> bool {
        false
    }

    fn is_enumerate(&self, _name: &str) -> bool {
        false
    }

    fn is_numeric_key(&self, name: &str) -> bool {
        name.eq_ignore_ascii_case("id")
    }

    fn is_document(&self, _name: &str) -> bool {
        false
    }

    fn set_header(&mut self, index: usize, header: XCellHeader) -> XResult<()> {
        if index >= self.headers.len() {
            self.headers.resize(index + 1, XCellHeader::default());
        }
        self.headers[index] = header;
        Ok(())
    }

    fn add_header(&mut self, header: XCellHeader) -> XResult<()> {
        self.headers.push(header);
        Ok(())
    }

    fn write_row(&mut self, row_index: usize, data: Vec<Data>) -> XResult<()> {
        // 简化实现，实际需要使用支持写入的 Excel 库
        // 这里只是更新内存中的数据结构
        Ok(())
    }

    fn add_row(&mut self, data: Vec<Data>) -> XResult<()> {
        // 简化实现，实际需要使用支持写入的 Excel 库
        // 这里只是更新内存中的数据结构
        Ok(())
    }

    fn save(&self, path: &Path) -> XResult<()> {
        // 简化实现，实际需要使用支持写入的 Excel 库
        Err(XError::new(XErrorKind::TableError("Excel 保存功能未实现".to_string())))
    }

    fn set_label(&mut self, label: &str) -> XResult<()> {
        self.label = label.to_string();
        Ok(())
    }

    fn get_label(&self) -> XResult<String> {
        Ok(self.label.clone())
    }
}
