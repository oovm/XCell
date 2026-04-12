use std::{path::Path, sync::Arc};
use xcell_provider::{TableReader as XCellTableReader, XCellHeader};
use xcell_core::{XError, XErrorKind, XResult};
use xcell_config::{ProjectConfig, TableConfig};

use super::*;

/// 表格读取器 trait，扩展 xcell_provider 的 TableReader
pub trait XTableReader: XCellTableReader {
    /// 加载表格文件
    fn load(path: &Path, config: &crate::ProjectConfig) -> XResult<Self>
    where
        Self: Sized;
}

/// 包装 Arc<dyn XTableReader> 的表格读取器
#[derive(Debug, Clone)]
pub struct ArcTableReader {
    inner: Arc<dyn XTableReader>,
}

impl ArcTableReader {
    /// 创建新的共享表格读取器
    pub fn new(inner: Arc<dyn XTableReader>) -> Self {
        Self { inner }
    }
}

impl XCellTableReader for ArcTableReader {
    fn load(_path: &Path) -> XResult<Self> {
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

    fn parse_type(&self, name: &str) -> xcell_core::XCellTyped {
        XCellTableReader::parse_type(&*self.inner, name)
    }

    fn parse_type_with_config(&self, name: &str, config: &xcell_core::TypeMetaInfo) -> xcell_core::XCellTyped {
        XCellTableReader::parse_type_with_config(&*self.inner, name, config)
    }

    fn default_enumerate(&self) -> xcell_core::IntegerKind {
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

impl XTableReader for ArcTableReader {
    fn load(path: &Path, config: &crate::ProjectConfig) -> XResult<Self> {
        let inner = load_table(path, config)?;
        Ok(Self::new(inner))
    }
}

/// 加载表格文件
///
/// # Parameters
/// - `path`: 表格文件路径
/// - `config`: 项目配置
///
/// # Returns
/// 表格读取器
pub fn load_table(path: &Path, config: &crate::ProjectConfig) -> XResult<Arc<dyn XTableReader>> {
    let table_config = try_load_table_config(path, config)?;
    let inner = xcell_provider::CalamineTable::load_with_full_config(
        path,
        &table_config.typing,
        &table_config.line,
        &table_config.fields,
    )?;
    let adapter = CalamineTableAdapter { inner, config: config.clone() };
    Ok(Arc::new(adapter))
}

/// 尝试加载表格配置
fn try_load_table_config(path: &Path, global: &ProjectConfig) -> XResult<TableConfig> {
    let file = path.with_extension("toml");
    let file = if file.exists() { Some(file.as_path()) } else { None };
    TableConfig::load_file(file, Some(global))
}

/// CalamineTable 适配器，实现 analyzer 的 XTableReader trait
struct CalamineTableAdapter {
    inner: xcell_provider::CalamineTable,
    config: crate::ProjectConfig,
}

impl std::fmt::Debug for CalamineTableAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CalamineTableAdapter").finish()
    }
}

impl XCellTableReader for CalamineTableAdapter {
    fn load(_path: &Path) -> XResult<Self> {
        Err(XError::new(XErrorKind::TableError("CalamineTableAdapter::load not implemented".to_string())))
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

    fn parse_type(&self, name: &str) -> xcell_core::XCellTyped {
        self.inner.parse_type_with_config(name, &self.config.typing)
    }

    fn parse_type_with_config(&self, name: &str, config: &xcell_core::TypeMetaInfo) -> xcell_core::XCellTyped {
        self.inner.parse_type_with_config(name, config)
    }

    fn default_enumerate(&self) -> xcell_core::IntegerKind {
        self.config.typing.enumerate.integer
    }

    fn is_language_define(&self) -> bool {
        let name = self.get_header(0);
        let norm = crate::utils::norm_string(&name.field_name);
        self.config.typing.language.is_id(&norm)
    }

    fn is_language_table(&self) -> bool {
        let name = self.get_header(0);
        let norm = crate::utils::norm_string(&name.field_name);
        self.config.typing.language.is_key(&norm)
    }

    fn is_language_value(&self, name: &str) -> bool {
        let norm = crate::utils::norm_string(name);
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

impl XTableReader for CalamineTableAdapter {
    fn load(path: &Path, config: &crate::ProjectConfig) -> XResult<Self> {
        let table_config = try_load_table_config(path, config)?;
        let inner = xcell_provider::CalamineTable::load_with_full_config(
            path,
            &table_config.typing,
            &table_config.line,
            &table_config.fields,
        )?;
        Ok(Self { inner, config: config.clone() })
    }
}
