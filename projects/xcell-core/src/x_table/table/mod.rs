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
            sum_excel: 0,
            sum_config: 0,
            errors: vec![],
        }
    }
}

impl XCellTable {
    /// 加载配置表
    ///
    /// # Arguments
    ///
    /// * `path`: Excel 路径
    /// * `global`: 全局设置
    ///
    /// returns: Result<XCellTable, XError>
    ///
    /// # Examples
    ///
    /// ```
    /// use xcell_core::XCellTable;
    /// ```
    pub fn load_file(path: PathBuf, global: &ProjectConfig) -> XResult<Self> {
        let mut xcell = Self::default();
        xcell.table = find_first_table(&path)?;
        xcell.read_headers()?;
        xcell.load_config(global)?;
        if xcell.check_sum_change() {}
        Ok(xcell)
    }
    pub fn check_sum_change(&mut self) -> bool {
        self.check_excel_change() || self.check_config_change()
    }
    pub fn check_excel_change(&mut self) -> bool {
        let sum = 0;
        let changed = self.sum_excel != sum;
        if changed {
            self.sum_excel = sum;
        }
        changed
    }
    pub fn check_config_change(&mut self) -> bool {
        let sum = 0;
        let changed = self.sum_excel != sum;
        if changed {
            self.sum_excel = sum;
        }
        changed
    }
    fn load_config(&mut self, global: &ProjectConfig) -> XResult<()> {
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
                self.headers.push(XCellHeader {
                    comment: data.to_string(),
                    column: i,
                    typing,
                    field_name,
                    details: "".to_string(),
                })
            }
        }
        Ok(())
    }
}
