use crate::utils::norm_string;
use super::*;

pub struct CalamineTableHeaders<'i> {
    table: &'i CalamineTable,
    this: usize,
    last: usize,
}

impl CalamineTable {
    pub fn load(path: &Path, config: &ProjectConfig) -> XResult<Self> {
        let path = path.canonicalize()?;
        let table = find_first_table(&path)?;
        let config = Self::try_load_config(&path, config)?;
        // let toml = config.get_table_config(&table)?;
        Ok(Self {
            path,
            table,
            config,
        })
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
            Some(s) => { s }
            None => {
                complete = false;
                Default::default()
            }
        };
        let typing = match self.get_field_type(index) {
            Some(s) => { s }
            None => {
                complete = false;
                Default::default()
            }
        };
        let (summary, details) = self.read_comment_details(index).unwrap_or_default();
        XCellHeader {
            column: index,
            summary,
            details,
            typing,
            field_name,
            complete,
        }
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
        let typing = XCellTyped::parse(value, &self.typing);
        Some(typing)
    }
    fn read_comment_details(&self, index: usize) -> Option<(String, String)> {
        let line = self.config.line.helper.saturating_sub(1) as u32;
        let comment = self.table.get_value((line, index as u32))?;
        let summary = comment.to_string();
        Some((summary, String::new()))
    }
}


impl CalamineTable {
    pub fn headers(&self) -> CalamineTableHeaders {
        let mut max_width = 0;
        for i in self.table.rows() {
            let width = i.len();
            if width > max_width {
                max_width = width;
            }
        }
        CalamineTableHeaders {
            table: &self,
            this: 0,
            last: max_width,
        }
    }
}

impl Iterator for CalamineTableHeaders {
    type Item = XCellHeader;

    fn next(&mut self) -> Option<Self::Item> {
        if self.this >= self.last {
            return None;
        }
        self.this += 1;
        Some(self.table.get_header(self.this))
    }
}