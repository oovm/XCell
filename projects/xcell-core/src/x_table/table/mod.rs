use crate::{utils::find_first_table, x_table::global_config::ProjectConfig};

use super::*;

impl Debug for XCellTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XCellTable")
            .field("path", &self.path)
            .field("config", &self.config)
            .field("headers", &self.headers)
            .field("errors", &self.errors)
            .finish()
    }
}

impl Default for XCellTable {
    fn default() -> Self {
        Self {
            path: Default::default(),
            table: Default::default(),
            headers: vec![],
            config: Default::default(),
            errors: vec![],
        }
    }
}

impl XCellTable {
    pub fn load_file(path: PathBuf, global: Option<&ProjectConfig>) -> XResult<Self> {
        let mut xcell = Self::default();
        xcell.table = find_first_table(&path)?;
        xcell.read_headers()?;
        xcell.load_config(global)?;
        Ok(xcell)
    }
    fn load_config(&mut self, _global: Option<&ProjectConfig>) -> XResult<()> {
        let mut dir = self.path.clone();
        let name = match self.path.file_stem() {
            None => "",
            Some(s) => match s.to_str() {
                None => "",
                Some(s) => s,
            },
        };
        dir.pop();
        let config = dir.join(format!("{}.toml", name));
        if config.exists() {
            self.config = TableConfig::load_file(&config)?;
        }
        Ok(())
    }

    fn read_headers(&mut self) -> XResult<()> {
        let row = match self.table.rows().nth(0) {
            Some(s) => s,
            None => return Err(XError::table_error("找不到描述, 表第一行格式非法")),
        };
        for (i, data) in row.iter().enumerate() {
            if !data.is_empty() {
                let typing = match self.table.get_value((1, i as u32)) {
                    Some(s) => XCellTyped::from(s),
                    None => continue,
                };
                let field_name = match self.table.get_value((2, i as u32)) {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                self.headers.push(XCellHeader { comment: data.to_string(), column: i, typing, field_name })
            }
        }
        Ok(())
    }
}
