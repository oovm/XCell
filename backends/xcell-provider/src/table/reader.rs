//! 表格读取器模块
//!
//! 提供统一的表格读取接口和各种格式的表格读取器实现。

use calamine::{Data, Reader};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use xcell_core::{FieldConfig, IntegerKind, TableLineMode, TypeMetaInfo, XCellAccess, XCellTyped, XDocument, XError, XErrorKind, XResult};
pub use xcell_parser::FieldConstraint;
use xcell_parser::{norm_string, parse_field, parse_meta, TableKind};

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

    /// 获取表格文件路径
    ///
    /// # Returns
    /// - 表格文件的路径
    fn get_path(&self) -> PathBuf;
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

    fn get_path(&self) -> PathBuf {
        self.path.clone()
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

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// Calamine 表格读取器，支持 Excel 等格式的完整解析
#[derive(Clone, Debug)]
pub struct CalamineTable {
    /// 表格的绝对路径
    path: PathBuf,
    /// 原始表单数据
    table: calamine::Range<Data>,
    /// 类型元信息
    typing: TypeMetaInfo,
    /// 行模式配置
    line: TableLineMode,
    /// 字段配置列表
    fields: Vec<FieldConfig>,
}

impl CalamineTable {
    /// 加载 Excel 表格文件并应用类型配置
    ///
    /// # Parameters
    /// - `path`: 表格文件的路径
    /// - `typing`: 类型元信息
    ///
    /// # Returns
    /// - 成功时返回 CalamineTable 实例
    /// - 失败时返回错误
    pub fn load_with_typing(path: &Path, typing: &TypeMetaInfo) -> XResult<Self> {
        let path = path.canonicalize().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let table = crate::find_first_table(&path)?;
        Ok(Self {
            path,
            table,
            typing: typing.clone(),
            line: TableLineMode::default(),
            fields: Vec::new(),
        })
    }

    /// 加载 Excel 表格文件并应用完整配置
    ///
    /// # Parameters
    /// - `path`: 表格文件的路径
    /// - `typing`: 类型元信息
    /// - `line`: 行模式配置
    /// - `fields`: 字段配置列表
    ///
    /// # Returns
    /// - 成功时返回 CalamineTable 实例
    /// - 失败时返回错误
    pub fn load_with_full_config(
        path: &Path,
        typing: &TypeMetaInfo,
        line: &TableLineMode,
        fields: &[FieldConfig],
    ) -> XResult<Self> {
        let path = path.canonicalize().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let table = crate::find_first_table(&path)?;
        Ok(Self {
            path,
            table,
            typing: typing.clone(),
            line: *line,
            fields: fields.to_vec(),
        })
    }

    /// 获取内部表格数据的引用
    pub fn inner_table(&self) -> &calamine::Range<Data> {
        &self.table
    }

    /// 获取行模式配置
    pub fn line_config(&self) -> &TableLineMode {
        &self.line
    }

    /// 获取表格类型
    ///
    /// # Returns
    /// - 表格类型标记，如果第一行第一列包含有效的元数据表达式
    pub fn table_kind(&self) -> Option<TableKind> {
        let meta_row_worksheet = 0;
        let meta_row_range = match self.table.start() {
            Some((start_row, _)) => meta_row_worksheet - start_row,
            None => meta_row_worksheet,
        };
        if let Some(value) = self.table.get_value((meta_row_range, 0)) {
            if let Data::String(s) = value {
                if let Ok(meta) = parse_meta(s) {
                    return Some(meta.kind);
                }
            }
        }
        None
    }

    /// 获取第 `index` 列的字段名和约束
    fn get_field_name(&self, index: usize) -> Option<(String, Option<FieldConstraint>)> {
        if let Some(field) = self.fields.get(index) {
            if !field.name.is_empty() {
                return Some((field.name.clone(), None));
            }
        }
        let field_row_worksheet = self.line.field as u32 - 1;
        let field_row_range = match self.table.start() {
            Some((start_row, _)) => field_row_worksheet - start_row,
            None => field_row_worksheet,
        };
        if let Some(value) = self.table.get_value((field_row_range, index as u32)) {
            if let Data::String(s) = value {
                if let Ok(field_expr) = parse_field(s) {
                    return Some((field_expr.name, field_expr.constraint));
                }
                return Some((s.to_string(), None));
            }
        }
        None
    }

    /// 获取第 `index` 列的字段类型
    fn get_field_type(&self, index: usize) -> Option<XCellTyped> {
        if let Some(field) = self.fields.get(index) {
            if !field.r#type.is_empty() {
                return Some(XCellTyped::parse(&field.r#type, &self.typing));
            }
        }
        let type_row_worksheet = self.line.r#type as u32 - 1;
        let type_row_range = match self.table.start() {
            Some((start_row, _)) => type_row_worksheet - start_row,
            None => type_row_worksheet,
        };
        if let Some(value) = self.table.get_value((type_row_range, index as u32)) {
            let type_str = match value {
                Data::String(s) => s.to_string(),
                Data::Int(i) => i.to_string(),
                Data::Float(f) => f.to_string(),
                Data::Bool(b) => b.to_string(),
                Data::DateTime(dt) => dt.to_string(),
                Data::DateTimeIso(dt) => dt.to_string(),
                Data::DurationIso(dur) => dur.to_string(),
                _ => return None,
            };
            if !type_str.is_empty() {
                let parsed = XCellTyped::parse(&type_str, &self.typing);
                tracing::debug!(
                    "解析类型: 列={}, 类型行={}, 类型字符串='{}', 解析结果={:?}",
                    index, type_row_range, type_str, parsed
                );
                return Some(parsed);
            }
        }
        None
    }

    /// 读取第 `index` 列的注释详情
    fn read_comment_details(&self, index: usize) -> XDocument {
        let comment_row = self.line.comment;
        if comment_row == 0 {
            return XDocument::default();
        }
        let comment_row_worksheet = comment_row as u32 - 1;
        let comment_row_range = match self.table.start() {
            Some((start_row, _)) => comment_row_worksheet - start_row,
            None => comment_row_worksheet,
        };
        if let Some(value) = self.table.get_value((comment_row_range, index as u32)) {
            let comment = match value {
                Data::String(s) => s.clone(),
                Data::Int(i) => i.to_string(),
                Data::Float(f) => f.to_string(),
                Data::Bool(b) => b.to_string(),
                Data::DateTime(dt) => dt.to_string(),
                _ => String::new(),
            };
            if !comment.is_empty() {
                return XDocument::new(comment);
            }
        }
        XDocument::default()
    }
}

impl TableReader for CalamineTable {
    fn load(path: &Path) -> XResult<Self> {
        let path = path.canonicalize().map_err(|e| XError::new(XErrorKind::IOError(e.to_string())))?;
        let table = crate::find_first_table(&path)?;
        Ok(Self {
            path,
            table,
            typing: TypeMetaInfo::default(),
            line: TableLineMode::default(),
            fields: Vec::new(),
        })
    }

    fn load_with_config(path: &Path, config: &TypeMetaInfo) -> XResult<Self> {
        Self::load_with_typing(path, config)
    }

    fn get_name(&self) -> String {
        self.path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string()
    }

    fn get_header(&self, index: usize) -> XCellHeader {
        let mut complete = true;
        let (field_name, constraint) = match self.get_field_name(index) {
            Some((name, constraint)) => (name, constraint),
            None => {
                complete = false;
                (Default::default(), None)
            }
        };
        let typing = match self.get_field_type(index) {
            Some(s) => s,
            None => {
                complete = false;
                Default::default()
            }
        };
        let access = if field_name.starts_with('_') { XCellAccess::Private } else { XCellAccess::Public };
        XCellHeader {
            column: index,
            document: self.read_comment_details(index),
            typing,
            field_name,
            complete,
            access,
            constraint,
        }
    }

    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_> {
        let width = self.table.width() as usize;
        Box::new((0..width).map(|i| self.get_header(i)))
    }

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<Data>)> + '_> {
        let range = self.table.clone();
        Box::new((0..range.height()).map(move |row| {
            let mut data = Vec::new();
            for col in 0..range.width() {
                if let Some(value) = range.get_value((row as u32, col as u32)) {
                    data.push(value.to_owned());
                } else {
                    data.push(Data::Empty);
                }
            }
            (row as usize, data)
        }))
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        XCellTyped::parse(name, &self.typing)
    }

    fn parse_type_with_config(&self, name: &str, config: &TypeMetaInfo) -> XCellTyped {
        XCellTyped::parse(name, config)
    }

    fn default_enumerate(&self) -> IntegerKind {
        self.typing.enumerate.integer
    }

    fn is_language_define(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.typing.language.is_id(&norm)
    }

    fn is_language_table(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.typing.language.is_key(&norm)
    }

    fn is_language_value(&self, name: &str) -> bool {
        let norm = norm_string(name);
        self.typing.language.is_value(&norm)
    }

    fn is_class(&self) -> bool {
        let name = self.get_header(0);
        name.field_name.eq_ignore_ascii_case("class")
    }

    fn is_list(&self) -> bool {
        let head = self.get_header(0);
        head.field_name.eq_ignore_ascii_case("id")
    }

    fn is_dict(&self) -> bool {
        let head = self.get_header(0);
        head.field_name.eq_ignore_ascii_case("key")
    }

    fn is_group(&self, name: &str) -> bool {
        self.typing.language.is_group(name)
    }

    fn is_enumerate(&self, name: &str) -> bool {
        name == "enum"
    }

    fn is_numeric_key(&self, name: &str) -> bool {
        name.eq_ignore_ascii_case("id")
    }

    fn is_document(&self, name: &str) -> bool {
        name == "document"
    }

    fn set_header(&mut self, _index: usize, _header: XCellHeader) -> XResult<()> {
        Ok(())
    }

    fn add_header(&mut self, _header: XCellHeader) -> XResult<()> {
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

    fn set_label(&mut self, _label: &str) -> XResult<()> {
        Ok(())
    }

    fn get_label(&self) -> XResult<String> {
        Ok(String::new())
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// 共享表格读取器，用于在多个消费者之间共享 `TableReader` 实例
#[derive(Debug, Clone)]
pub struct ArcTableReader {
    inner: Arc<dyn TableReader>,
}

impl ArcTableReader {
    /// 创建新的共享表格读取器
    ///
    /// # Parameters
    /// - `inner`: 被包装的表格读取器
    pub fn new(inner: Arc<dyn TableReader>) -> Self {
        Self { inner }
    }
}

impl TableReader for ArcTableReader {
    fn load(_path: &Path) -> XResult<Self> {
        Err(XError::new(XErrorKind::TableError("ArcTableReader::load not implemented".to_string())))
    }

    fn load_with_config(_path: &Path, _config: &TypeMetaInfo) -> XResult<Self> {
        Err(XError::new(XErrorKind::TableError("ArcTableReader::load_with_config not implemented".to_string())))
    }

    fn get_name(&self) -> String {
        self.inner.get_name()
    }

    fn get_header(&self, index: usize) -> XCellHeader {
        self.inner.get_header(index)
    }

    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_> {
        self.inner.headers()
    }

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<Data>)> + '_> {
        self.inner.rows()
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        self.inner.parse_type(name)
    }

    fn parse_type_with_config(&self, name: &str, config: &TypeMetaInfo) -> XCellTyped {
        self.inner.parse_type_with_config(name, config)
    }

    fn default_enumerate(&self) -> IntegerKind {
        self.inner.default_enumerate()
    }

    fn is_language_define(&self) -> bool {
        self.inner.is_language_define()
    }

    fn is_language_table(&self) -> bool {
        self.inner.is_language_table()
    }

    fn is_language_value(&self, name: &str) -> bool {
        self.inner.is_language_value(name)
    }

    fn is_class(&self) -> bool {
        self.inner.is_class()
    }

    fn is_list(&self) -> bool {
        self.inner.is_list()
    }

    fn is_dict(&self) -> bool {
        self.inner.is_dict()
    }

    fn is_group(&self, name: &str) -> bool {
        self.inner.is_group(name)
    }

    fn is_enumerate(&self, name: &str) -> bool {
        self.inner.is_enumerate(name)
    }

    fn is_numeric_key(&self, name: &str) -> bool {
        self.inner.is_numeric_key(name)
    }

    fn is_document(&self, name: &str) -> bool {
        self.inner.is_document(name)
    }

    fn set_header(&mut self, _index: usize, _header: XCellHeader) -> XResult<()> {
        Ok(())
    }

    fn add_header(&mut self, _header: XCellHeader) -> XResult<()> {
        Ok(())
    }

    fn write_row(&mut self, _row_index: usize, _data: Vec<Data>) -> XResult<()> {
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<Data>) -> XResult<()> {
        Ok(())
    }

    fn save(&self, path: &Path) -> XResult<()> {
        self.inner.save(path)
    }

    fn set_label(&mut self, _label: &str) -> XResult<()> {
        Ok(())
    }

    fn get_label(&self) -> XResult<String> {
        self.inner.get_label()
    }

    fn get_path(&self) -> PathBuf {
        self.inner.get_path()
    }
}

/// 将 calamine::Data 转换为 xcell_core::for_3rd::Data
///
/// # Parameters
/// - `data`: calamine 的单元格数据
///
/// # Returns
/// - 转换后的 xcell_core 数据
pub fn convert_data(data: &Data) -> xcell_core::for_3rd::Data {
    match data {
        Data::Int(v) => xcell_core::for_3rd::Data::Int(*v),
        Data::Float(v) => xcell_core::for_3rd::Data::Float(*v),
        Data::String(v) => xcell_core::for_3rd::Data::String(v.clone()),
        Data::Bool(v) => xcell_core::for_3rd::Data::Bool(*v),
        Data::DateTime(v) => xcell_core::for_3rd::Data::DateTime(v.as_f64()),
        Data::DateTimeIso(v) => xcell_core::for_3rd::Data::DateTimeIso(v.clone()),
        Data::DurationIso(v) => xcell_core::for_3rd::Data::DurationIso(v.clone()),
        Data::Error(v) => xcell_core::for_3rd::Data::Error(format!("{:?}", v)),
        Data::Empty => xcell_core::for_3rd::Data::Empty,
    }
}
