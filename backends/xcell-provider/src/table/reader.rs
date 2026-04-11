//! 表格读取器模块
//!
//! 提供统一的表格读取接口和各种格式的表格读取器实现。

use calamine::{Data, Reader};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use xcell_core::{IntegerKind, TypeMetaInfo, XCellAccess, XCellTyped, XDocument, XError, XErrorKind, XResult};
pub use xcell_parser::FieldConstraint;

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
    /// 字段约束
    pub constraint: Option<FieldConstraint>,
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
        Ok(())
    }

    /// 添加表头
    ///
    /// # Parameters
    /// - `header`: 表头信息
    fn add_header(&mut self, header: XCellHeader) -> XResult<()> {
        Ok(())
    }

    /// 写入行数据
    ///
    /// # Parameters
    /// - `row_index`: 行索引
    /// - `data`: 行数据
    fn write_row(&mut self, row_index: usize, data: Vec<Data>) -> XResult<()> {
        Ok(())
    }

    /// 添加行数据
    ///
    /// # Parameters
    /// - `data`: 行数据
    fn add_row(&mut self, data: Vec<Data>) -> XResult<()> {
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
        Ok(())
    }

    /// 获取表格标签
    ///
    /// # Returns
    /// - 标签名称
    fn get_label(&self) -> XResult<String> {
        Ok(String::new())
    }
}

/// CSV 行流式迭代器
///
/// 从内存缓冲区中逐行读取 CSV 数据，避免一次性加载所有行到内存。
pub struct CsvRows {
    /// CSV 读取器
    reader: csv::Reader<Cursor<Vec<u8>>>,
    /// 当前行索引
    row_index: usize,
}

impl Iterator for CsvRows {
    type Item = (usize, Vec<Data>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.records().next() {
                Some(Ok(record)) => {
                    let mut data = Vec::with_capacity(record.len());
                    for s in record.iter() {
                        data.push(Data::String(s.to_string()));
                    }
                    let index = self.row_index;
                    self.row_index += 1;
                    return Some((index, data));
                }
                Some(Err(_)) => {
                    self.row_index += 1;
                    continue;
                }
                None => return None,
            }
        }
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
    /// 文件内容缓冲区
    content: Vec<u8>,
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
        let content = std::fs::read(&path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut headers = Vec::new();

        let mut reader = csv::Reader::from_reader(Cursor::new(&content));
        if let Ok(header_row) = reader.headers() {
            for (i, field_name) in header_row.iter().enumerate() {
                headers.push(XCellHeader {
                    column: i,
                    access: XCellAccess::Public,
                    field_name: field_name.to_string(),
                    typing: XCellTyped::default(),
                    document: XDocument::default(),
                    complete: true,
                    constraint: None,
                });
            }
        }

        Ok(Self { path, label: String::new(), headers, content })
    }

    /// 创建 CSV 读取器
    ///
    /// 从内存缓冲区创建 CSV 读取器，避免重复读取磁盘文件。
    fn create_reader(&self) -> csv::Reader<Cursor<&[u8]>> {
        csv::Reader::from_reader(Cursor::new(&self.content))
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
        let cursor = Cursor::new(self.content.clone());
        let reader = csv::Reader::from_reader(cursor);
        Box::new(CsvRows { reader, row_index: 0 })
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        let info = TypeMetaInfo::default();
        XCellTyped::parse(name, &info)
    }

    fn default_enumerate(&self) -> IntegerKind {
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
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<Data>) -> XResult<()> {
        Ok(())
    }

    fn save(&self, path: &Path) -> XResult<()> {
        let file = std::fs::File::create(path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut writer = csv::Writer::from_writer(file);

        if !self.headers.is_empty() {
            let header_row: Vec<String> = self.headers.iter().map(|h| h.field_name.clone()).collect();
            writer.write_record(&header_row).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        }

        let mut reader = self.create_reader();
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

/// TSV 行流式迭代器
///
/// 从内存缓冲区中逐行读取 TSV 数据，避免一次性加载所有行到内存。
pub struct TsvRows {
    /// TSV 读取器
    reader: csv::Reader<Cursor<Vec<u8>>>,
    /// 当前行索引
    row_index: usize,
}

impl Iterator for TsvRows {
    type Item = (usize, Vec<Data>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.records().next() {
                Some(Ok(record)) => {
                    let mut data = Vec::with_capacity(record.len());
                    for s in record.iter() {
                        data.push(Data::String(s.to_string()));
                    }
                    let index = self.row_index;
                    self.row_index += 1;
                    return Some((index, data));
                }
                Some(Err(_)) => {
                    self.row_index += 1;
                    continue;
                }
                None => return None,
            }
        }
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
    /// 文件内容缓冲区
    content: Vec<u8>,
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
        let content = std::fs::read(&path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut headers = Vec::new();

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(Cursor::new(&content));

        if let Ok(header_row) = reader.headers() {
            for (i, field_name) in header_row.iter().enumerate() {
                headers.push(XCellHeader {
                    column: i,
                    access: XCellAccess::Public,
                    field_name: field_name.to_string(),
                    typing: XCellTyped::default(),
                    document: XDocument::default(),
                    complete: true,
                    constraint: None,
                });
            }
        }

        Ok(Self { path, label: String::new(), headers, content })
    }

    /// 创建 TSV 读取器
    ///
    /// 从内存缓冲区创建 TSV 读取器，避免重复读取磁盘文件。
    fn create_reader(&self) -> csv::Reader<Cursor<&[u8]>> {
        csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(Cursor::new(&self.content))
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
        let cursor = Cursor::new(self.content.clone());
        let reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(cursor);
        Box::new(TsvRows { reader, row_index: 0 })
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        let info = TypeMetaInfo::default();
        XCellTyped::parse(name, &info)
    }

    fn default_enumerate(&self) -> IntegerKind {
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
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<Data>) -> XResult<()> {
        Ok(())
    }

    fn save(&self, path: &Path) -> XResult<()> {
        let file = std::fs::File::create(path).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let mut writer = csv::WriterBuilder::new().delimiter(b'\t').from_writer(file);

        if !self.headers.is_empty() {
            let header_row: Vec<String> = self.headers.iter().map(|h| h.field_name.clone()).collect();
            writer.write_record(&header_row).map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        }

        let mut reader = self.create_reader();
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

        // 解析表头信息，默认从第 2 行开始（1-based）
        let mut headers = Vec::new();
        let field_row: u32 = 1; // 0-based，对应 1-based 的第 2 行
        
        if (range.height() as u32) > field_row {
            for col in 0..(range.width() as u32) {
                if let Some(value) = range.get_value((field_row, col)) {
                    let field_name = match value {
                        Data::String(s) => s.to_string(),
                        _ => String::new(),
                    };
                    
                    headers.push(XCellHeader {
                        column: col as usize,
                        access: XCellAccess::Public,
                        field_name,
                        typing: XCellTyped::default(),
                        document: XDocument::default(),
                        complete: true,
                        constraint: None,
                    });
                }
            }
        }

        Ok(Self { path, table: range.clone(), label: String::new(), headers, sheet_name: first_sheet.to_string() })
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

    fn parse_type(&self, name: &str) -> XCellTyped {
        let info = TypeMetaInfo::default();
        XCellTyped::parse(name, &info)
    }

    fn default_enumerate(&self) -> IntegerKind {
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
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<Data>) -> XResult<()> {
        Ok(())
    }

    fn save(&self, _path: &Path) -> XResult<()> {
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
