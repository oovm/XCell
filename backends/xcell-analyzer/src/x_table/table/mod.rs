use crate::utils::norm_string;
use calamine::Data;
use std::{path::Path, sync::Arc};
use xcell_parser::{parse_field, parse_meta, FieldConstraint, TableKind};
use xcell_provider::{TableReader as XCellTableReader, XCellAccess, XCellHeader, XDocument, load_table as load_table_reader};
use xcell_core::{ByteOrder, IntegerKind, StreamReader, XCellTyped, XErrorKind};

use super::*;

mod display;
mod headers;
mod rows;

/// 表格读取器 trait
pub trait TableReader: XCellTableReader {
    /// 加载表格文件
    fn load(path: &Path, config: &crate::ProjectConfig) -> XResult<Self>
    where
        Self: Sized;

    /// 解析类型
    fn parse_type(&self, name: &str) -> XCellTyped;

    /// 获取默认枚举类型
    fn default_enumerate(&self) -> IntegerKind;

    /// 检查是否为语言定义表
    fn is_language_define(&self) -> bool;

    /// 检查是否为语言表
    fn is_language_table(&self) -> bool;

    /// 检查是否为语言值
    fn is_language_value(&self, name: &str) -> bool;

    /// 检查是否为类表
    fn is_class(&self) -> bool;

    /// 检查是否为列表
    fn is_list(&self) -> bool;

    /// 检查是否为字典表
    fn is_dict(&self) -> bool;

    /// 检查是否为组
    fn is_group(&self, name: &str) -> bool;

    /// 检查是否为枚举
    fn is_enumerate(&self, name: &str) -> bool;

    /// 检查是否为数字键
    fn is_numeric_key(&self, name: &str) -> bool;

    /// 检查是否为文档
    fn is_document(&self, name: &str) -> bool;
}

/// 包装 xcell-provider 的表格读取器
#[derive(Debug)]
pub struct WrappedTableReader {
    inner: Box<dyn XCellTableReader>,
    config: crate::ProjectConfig,
}

/// 包装 Arc<dyn TableReader> 的表格读取器
#[derive(Debug, Clone)]
pub struct ArcTableReader {
    inner: Arc<dyn TableReader>,
}

impl ArcTableReader {
    pub fn new(inner: Arc<dyn TableReader>) -> Self {
        Self { inner }
    }
}

impl XCellTableReader for ArcTableReader {
    fn load(path: &Path) -> XResult<Self> {
        Err(XError::new(XErrorKind::TableError("ArcTableReader::load not implemented".to_string())))
    }

    fn get_name(&self) -> String {
        XCellTableReader::get_name(&*self.inner)
    }

    fn get_header(&self, index: usize) -> XCellHeader {
        XCellTableReader::get_header(&*self.inner, index)
    }

    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_> {
        XCellTableReader::headers(&*self.inner)
    }

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<calamine::Data>)> + '_> {
        XCellTableReader::rows(&*self.inner)
    }

    fn set_header(&mut self, _index: usize, _header: XCellHeader) -> XResult<()> {
        // 暂时不实现，因为 Arc<dyn TableReader> 不可变
        Ok(())
    }

    fn add_header(&mut self, _header: XCellHeader) -> XResult<()> {
        // 暂时不实现，因为 Arc<dyn TableReader> 不可变
        Ok(())
    }

    fn write_row(&mut self, _row: usize, _data: Vec<calamine::Data>) -> XResult<()> {
        // 暂时不实现，因为 Arc<dyn TableReader> 不可变
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<calamine::Data>) -> XResult<()> {
        // 暂时不实现，因为 Arc<dyn TableReader> 不可变
        Ok(())
    }

    fn save(&self, path: &std::path::Path) -> XResult<()> {
        XCellTableReader::save(&*self.inner, path)
    }

    fn set_label(&mut self, _label: &str) -> XResult<()> {
        // 暂时不实现，因为 Arc<dyn TableReader> 不可变
        Ok(())
    }

    fn get_label(&self) -> XResult<String> {
        XCellTableReader::get_label(&*self.inner)
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        XCellTableReader::parse_type(&*self.inner, name)
    }

    fn parse_type_with_config(&self, name: &str, config: &xcell_core::TypeMetaInfo) -> XCellTyped {
        XCellTableReader::parse_type_with_config(&*self.inner, name, config)
    }

    fn default_enumerate(&self) -> IntegerKind {
        XCellTableReader::default_enumerate(&*self.inner)
    }

    fn is_language_define(&self) -> bool {
        XCellTableReader::is_language_define(&*self.inner)
    }

    fn is_language_table(&self) -> bool {
        XCellTableReader::is_language_table(&*self.inner)
    }

    fn is_language_value(&self, name: &str) -> bool {
        XCellTableReader::is_language_value(&*self.inner, name)
    }

    fn is_class(&self) -> bool {
        XCellTableReader::is_class(&*self.inner)
    }

    fn is_list(&self) -> bool {
        XCellTableReader::is_list(&*self.inner)
    }

    fn is_dict(&self) -> bool {
        XCellTableReader::is_dict(&*self.inner)
    }

    fn is_group(&self, name: &str) -> bool {
        XCellTableReader::is_group(&*self.inner, name)
    }

    fn is_enumerate(&self, name: &str) -> bool {
        XCellTableReader::is_enumerate(&*self.inner, name)
    }

    fn is_numeric_key(&self, name: &str) -> bool {
        XCellTableReader::is_numeric_key(&*self.inner, name)
    }

    fn is_document(&self, name: &str) -> bool {
        XCellTableReader::is_document(&*self.inner, name)
    }
}

impl TableReader for ArcTableReader {
    fn load(path: &Path, config: &crate::ProjectConfig) -> XResult<Self> {
        let inner = load_table(path, config)?;
        Ok(Self::new(inner))
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        crate::x_table::table::TableReader::parse_type(&*self.inner, name)
    }

    fn default_enumerate(&self) -> IntegerKind {
        crate::x_table::table::TableReader::default_enumerate(&*self.inner)
    }

    fn is_language_define(&self) -> bool {
        crate::x_table::table::TableReader::is_language_define(&*self.inner)
    }

    fn is_language_table(&self) -> bool {
        crate::x_table::table::TableReader::is_language_table(&*self.inner)
    }

    fn is_language_value(&self, name: &str) -> bool {
        crate::x_table::table::TableReader::is_language_value(&*self.inner, name)
    }

    fn is_class(&self) -> bool {
        crate::x_table::table::TableReader::is_class(&*self.inner)
    }

    fn is_list(&self) -> bool {
        crate::x_table::table::TableReader::is_list(&*self.inner)
    }

    fn is_dict(&self) -> bool {
        crate::x_table::table::TableReader::is_dict(&*self.inner)
    }

    fn is_group(&self, name: &str) -> bool {
        crate::x_table::table::TableReader::is_group(&*self.inner, name)
    }

    fn is_enumerate(&self, name: &str) -> bool {
        crate::x_table::table::TableReader::is_enumerate(&*self.inner, name)
    }

    fn is_numeric_key(&self, name: &str) -> bool {
        crate::x_table::table::TableReader::is_numeric_key(&*self.inner, name)
    }

    fn is_document(&self, name: &str) -> bool {
        crate::x_table::table::TableReader::is_document(&*self.inner, name)
    }
}

impl XCellTableReader for WrappedTableReader {
    fn load(path: &Path) -> XResult<Self> {
        Err(XError::new(XErrorKind::TableError("WrappedTableReader::load not implemented".to_string())))
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

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<calamine::Data>)> + '_> {
        self.inner.rows()
    }

    fn set_header(&mut self, index: usize, header: XCellHeader) -> XResult<()> {
        self.inner.set_header(index, header)
    }

    fn add_header(&mut self, header: XCellHeader) -> XResult<()> {
        self.inner.add_header(header)
    }

    fn write_row(&mut self, row: usize, data: Vec<calamine::Data>) -> XResult<()> {
        self.inner.write_row(row, data)
    }

    fn add_row(&mut self, data: Vec<calamine::Data>) -> XResult<()> {
        self.inner.add_row(data)
    }

    fn save(&self, path: &std::path::Path) -> XResult<()> {
        self.inner.save(path)
    }

    fn set_label(&mut self, label: &str) -> XResult<()> {
        self.inner.set_label(label)
    }

    fn get_label(&self) -> XResult<String> {
        self.inner.get_label()
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        self.inner.parse_type(name)
    }

    fn parse_type_with_config(&self, name: &str, config: &xcell_core::TypeMetaInfo) -> XCellTyped {
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
}

impl WrappedTableReader {
    /// 加载表格文件
    pub fn new(path: &Path, config: &crate::ProjectConfig) -> XResult<Self> {
        let inner = load_table_reader(path)?;
        Ok(Self { inner, config: config.clone() })
    }
}

impl TableReader for WrappedTableReader {
    fn load(path: &Path, config: &crate::ProjectConfig) -> XResult<Self> {
        Self::new(path, config)
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        self.inner.parse_type_with_config(name, &self.config.typing)
    }

    fn default_enumerate(&self) -> IntegerKind {
        self.config.typing.enumerate.integer
    }

    fn is_language_define(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_id(&norm)
    }

    fn is_language_table(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_key(&norm)
    }

    fn is_language_value(&self, name: &str) -> bool {
        let norm = norm_string(name);
        self.config.typing.language.is_value(&norm)
    }

    fn is_class(&self) -> bool {
        let name = self.get_header(0);
        name.field_name.eq_ignore_ascii_case("class")
    }

    fn is_list(&self) -> bool {
        let head = self.get_header(0);
        <Self as TableReader>::is_numeric_key(self, &head.field_name)
    }

    fn is_dict(&self) -> bool {
        // 根据 dict.md 文档，默认即为 Dict 类型，无需显式标记
        // 只要不是 list 表，就默认是 dict 表
        !crate::x_table::table::TableReader::is_list(self)
    }

    fn is_group(&self, name: &str) -> bool {
        self.config.typing.language.is_group(name)
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
}

/// 为 `Arc<dyn TableReader>` 实现 `TableReader` trait
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CalamineTable {
    /// 表格的绝对路径
    path: PathBuf,
    /// 原始表单
    table: calamine::Range<Data>,
    /// 表单的配置
    config: TableConfig,
}

impl XCellTableReader for CalamineTable {
    fn load(_path: &Path) -> XResult<Self> {
        Err(XError::new(XErrorKind::TableError("CalamineTable::load not implemented".to_string())))
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
        let access = if field_name.starts_with("_") { XCellAccess::Private } else { XCellAccess::Public };
        XCellHeader { column: index, document: self.read_comment_details(index), typing, field_name, complete, access, constraint }
    }

    fn headers(&self) -> Box<dyn Iterator<Item = XCellHeader> + '_> {
        let width = self.table.width() as usize;
        Box::new((0..width).map(|i| self.get_header(i)))
    }

    fn rows(&self) -> Box<dyn Iterator<Item = (usize, Vec<calamine::Data>)> + '_> {
        let range = self.table.clone();
        Box::new((0..range.height()).map(move |row| {
            let mut data = Vec::new();
            for col in 0..range.width() {
                if let Some(value) = range.get_value((row as u32, col as u32)) {
                    data.push(value.to_owned());
                }
                else {
                    data.push(calamine::Data::Empty);
                }
            }
            (row as usize, data)
        }))
    }

    fn set_header(&mut self, _index: usize, _header: XCellHeader) -> XResult<()> {
        // 暂时不实现
        Ok(())
    }

    fn add_header(&mut self, _header: XCellHeader) -> XResult<()> {
        // 暂时不实现
        Ok(())
    }

    fn write_row(&mut self, _row: usize, _data: Vec<calamine::Data>) -> XResult<()> {
        // 暂时不实现
        Ok(())
    }

    fn add_row(&mut self, _data: Vec<calamine::Data>) -> XResult<()> {
        // 暂时不实现
        Ok(())
    }

    fn save(&self, _path: &std::path::Path) -> XResult<()> {
        // 暂时不实现
        Ok(())
    }

    fn set_label(&mut self, _label: &str) -> XResult<()> {
        // 暂时不实现
        Ok(())
    }

    fn get_label(&self) -> XResult<String> {
        // 暂时不实现
        Ok("".to_string())
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        XCellTyped::parse(name, &self.config.typing)
    }

    fn parse_type_with_config(&self, name: &str, config: &xcell_core::TypeMetaInfo) -> XCellTyped {
        XCellTyped::parse(name, config)
    }

    fn default_enumerate(&self) -> IntegerKind {
        self.config.typing.enumerate.integer
    }

    fn is_language_define(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_id(&norm)
    }

    fn is_language_table(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_key(&norm)
    }

    fn is_language_value(&self, name: &str) -> bool {
        let norm = norm_string(name);
        self.config.typing.language.is_value(&norm)
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
        self.config.typing.language.is_group(name)
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
}

impl CalamineTable {
    pub fn is_language_define(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_id(&norm)
    }

    pub fn is_language_table(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_key(&norm)
    }
    pub fn is_language_value(&self, name: &str) -> bool {
        let norm = norm_string(name);
        self.config.typing.language.is_value(&norm)
    }

    pub fn is_class(&self) -> bool {
        let name = self.get_header(0);
        name.field_name.eq_ignore_ascii_case("class")
    }

    pub fn is_list(&self) -> bool {
        let head = self.get_header(0);
        self.is_numeric_key(&head.field_name)
    }

    pub fn is_dict(&self) -> bool {
        // 根据 dict.md 文档，默认即为 Dict 类型，无需显式标记
        // 只要不是 list 表，就默认是 dict 表
        !<Self as TableReader>::is_list(self)
    }

    pub fn is_group(&self, name: &str) -> bool {
        self.config.typing.language.is_group(name)
    }

    pub fn is_enumerate(&self, name: &str) -> bool {
        name == "enum"
    }

    pub fn is_numeric_key(&self, name: &str) -> bool {
        name.eq_ignore_ascii_case("id")
    }

    pub fn is_document(&self, name: &str) -> bool {
        name == "document"
    }

    pub fn default_enumerate(&self) -> IntegerKind {
        self.config.typing.enumerate.integer
    }
}



impl CalamineTable {
    fn try_load_config(path: &Path, global: &ProjectConfig) -> XResult<TableConfig> {
        let file = path.with_extension("toml");
        let file = if file.exists() { Some(file.as_path()) } else { None };
        TableConfig::load_file(file, Some(global))
    }
}

impl CalamineTable {
    /// 获得第 `index` 列的表头
    pub fn get_header(&self, index: usize) -> XCellHeader {
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
        let access = if field_name.starts_with("_") { XCellAccess::Private } else { XCellAccess::Public };
        XCellHeader { column: index, document: self.read_comment_details(index), typing, field_name, complete, access, constraint }
    }
    
    /// 获取表格类型
    pub fn table_kind(&self) -> Option<TableKind> {
        // 表格类型标记通常在第一行第一列
        let meta_row_worksheet = 0; // 0-based worksheet row
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
    
    fn get_field_name(&self, index: usize) -> Option<(String, Option<FieldConstraint>)> {
        // 优先从 TOML 配置文件中获取字段名
        if let Some(field) = self.config.fields.get(index) {
            if !field.name.is_empty() {
                return Some((field.name.clone(), None));
            }
        }
        // 如果 TOML 配置文件中没有字段名，则从表格的 field 行获取
        let field_row_worksheet = self.config.line.field as u32 - 1; // 转换为 0-based worksheet row
        let field_row_range = match self.table.start() {
            Some((start_row, _)) => field_row_worksheet - start_row,
            None => field_row_worksheet,
        };
        if let Some(value) = self.table.get_value((field_row_range, index as u32)) {
            if let Data::String(s) = value {
                // 使用 xcell-parser 解析字段名
                if let Ok(field_expr) = parse_field(s) {
                    return Some((field_expr.name, field_expr.constraint));
                }
                // 回退到原始逻辑
                return Some((s.to_string(), None));
            }
        }
        None
    }
    fn get_field_type(&self, index: usize) -> Option<XCellTyped> {
        // 优先从 TOML 配置文件中获取字段类型
        if let Some(field) = self.config.fields.get(index) {
            if !field.r#type.is_empty() {
                return Some(XCellTyped::parse(&field.r#type, &self.config.typing));
            }
        }
        // 如果 TOML 配置文件中没有字段类型，则从表格的 type 行获取
        let type_row_worksheet = self.config.line.r#type as u32 - 1; // 转换为 0-based worksheet row
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
                return Some(XCellTyped::parse(&type_str, &self.config.typing));
            }
        }
        None
    }
    fn read_comment_details(&self, _index: usize) -> XDocument {
        XDocument::default()
    }
}

impl TableReader for CalamineTable {
    fn load(path: &Path, config: &crate::ProjectConfig) -> XResult<Self> {
        let path = path.canonicalize()?;
        let table = find_first_table(&path)?;
        let config = Self::try_load_config(&path, config)?;
        Ok(Self { path, table, config })
    }

    fn parse_type(&self, name: &str) -> XCellTyped {
        XCellTyped::parse(name, &self.config.typing)
    }

    fn default_enumerate(&self) -> IntegerKind {
        self.config.typing.enumerate.integer
    }

    fn is_language_define(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_id(&norm)
    }

    fn is_language_table(&self) -> bool {
        let name = self.get_header(0);
        let norm = norm_string(&name.field_name);
        self.config.typing.language.is_key(&norm)
    }

    fn is_language_value(&self, name: &str) -> bool {
        let norm = norm_string(name);
        self.config.typing.language.is_value(&norm)
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
        self.config.typing.language.is_group(name)
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
}

/// 加载表格文件
///
/// # Arguments
/// * `path` - 表格文件路径
/// * `config` - 项目配置
///
/// # Returns
/// 表格读取器
pub fn load_table(path: &Path, config: &crate::ProjectConfig) -> XResult<Arc<dyn TableReader>> {
    let table = <CalamineTable as TableReader>::load(path, config)?;
    Ok(Arc::new(table))
}
