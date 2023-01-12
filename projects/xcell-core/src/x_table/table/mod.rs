use super::*;

mod display;
mod headers;
mod rows;

#[derive(Clone, Debug)]
pub struct CalamineTable {
    /// 表格的绝对路径
    path: PathBuf,
    /// 原始表单
    table: calamine::Range<DataType>,
    /// 表单的配置
    config: TableConfig,
}

impl CalamineTable {
    #[inline]
    pub fn get_name(&self) -> String {
        self.path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string()
    }
    #[inline]
    pub fn is_language_id(&self) -> bool {
        let name = self.get_header(0).field_name.as_str();
        self.config.typing.language.is_id(name)
    }
    #[inline]
    pub fn is_language_key(&self) -> bool {
        let name = self.get_header(0).field_name.as_str();
        self.config.typing.language.is_key(name)
    }
    pub fn is_language_value(&self, name: &str) -> bool {
        self.config.typing.language.is_value(name)
    }
    #[inline]
    pub fn is_class(&self) -> bool {
        let name = self.get_header(0).field_name.as_str();
        name == "class"
    }
    #[inline]
    pub fn is_array(&self, name: &str) -> bool {
        name == "id"
    }
    #[inline]
    pub fn is_group(&self, name: &str) -> bool {
        self.config.typing.language.is_group(name)
    }
    #[inline]
    pub fn is_enumerate(&self, name: &str) -> bool {
        name == "enum"
    }
    #[inline]
    pub fn is_enumerate_id(&self, name: &str) -> bool {
        name == "id"
    }
    #[inline]
    pub fn is_document(&self, name: &str) -> bool {
        name == "document"
    }
}

impl CalamineTable {
    pub fn parse_type(&self, name: &str) -> XCellTyped {
        XCellTyped::parse(name, &self.config.typing)
    }
}

impl CalamineTable {
    pub fn load(path: &Path, config: &ProjectConfig) -> XResult<Self> {
        let path = path.canonicalize()?;
        let table = find_first_table(&path)?;
        let config = Self::try_load_config(&path, config)?;
        // let toml = config.get_table_config(&table)?;
        Ok(Self { path, table, config })
    }
    fn try_load_config(path: &Path, global: &ProjectConfig) -> XResult<TableConfig> {
        let file = path.with_extension("toml");
        let file = if file.exists() { Some(file.as_path()) } else { None };
        TableConfig::load_file(file, Some(global))
    }
}

impl CalamineTable {
    pub fn get_header(&self, index: usize) -> XCellHeader {
        let mut complete = true;
        let field_name = match self.get_field_name(index) {
            Some(s) => s,
            None => {
                complete = false;
                Default::default()
            }
        };
        let typing = match self.get_field_type(index) {
            Some(s) => s,
            None => {
                complete = false;
                Default::default()
            }
        };
        XCellHeader { column: index, comment: self.read_comment_details(index), typing, field_name, complete }
    }
    fn get_field_name(&self, index: usize) -> Option<String> {
        let line = self.config.line.field.saturating_sub(1) as u32;
        let value = self.table.get_value((line, index as u32))?.get_string()?;
        if value.is_empty() {
            return None;
        }
        Some(value.to_string())
    }
    fn get_field_type(&self, index: usize) -> Option<XCellTyped> {
        let line = self.config.line.typing.saturating_sub(1) as u32;
        let value = self.table.get_value((line, index as u32))?.get_string()?;
        if value.is_empty() {
            return None;
        }
        let typing = XCellTyped::parse(value, &self.config.typing);
        Some(typing)
    }
    fn read_comment_details(&self, index: usize) -> XDocument {
        let line = self.config.line.helper.saturating_sub(1) as u32;
        match self.table.get_value((line, index as u32)) {
            Some(s) => XDocument::from(s),
            None => XDocument::default(),
        }
    }
}
