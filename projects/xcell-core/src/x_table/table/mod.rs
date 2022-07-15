use crate::config::ProjectConfig;

use super::*;

mod display;

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
    pub fn load_file(path: &Path, global: &ProjectConfig) -> XResult<Self> {
        let mut xcell = Self {
            //
            path: Default::default(),
            name: "".to_string(),
            headers: Default::default(),
            data: Array2D::filled_with(XCellValue::Boolean(false), 1, 1),
            config: TableConfig::load_file(&path, Some(global))?,
            sum_excel: 0,
            sum_config: 0,
        };
        xcell.set_path(path)?;
        xcell.load_config(global)?;
        if xcell.check_sum_change() {
            match xcell.load_data() {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{e}")
                }
            }
        }
        Ok(xcell)
    }
    /// 强制重新加载表格中的数据
    pub fn load_data(&mut self) -> XResult<()> {
        let table = find_first_table(&self.path)?;
        let data = read_table_data(&table, &self.headers, &self.path)?;
        self.data = data;
        Ok(())
    }
    pub fn id(&self) -> u64 {
        xx_hash(self)
    }
    /// 检测是否要重新加载表格
    pub fn check_sum_change(&mut self) -> bool {
        self.check_excel_change() || self.check_config_change()
    }
    /// 检测表格是否发生变化
    pub fn check_excel_change(&mut self) -> bool {
        let sum = match xx_file(&self.path) {
            Ok(o) => o,
            Err(_) => return false,
        };
        let changed = self.sum_excel != sum;
        if changed {
            self.sum_excel = sum;
        }
        changed
    }
    /// 检查配置是否发生变化
    pub fn check_config_change(&mut self) -> bool {
        let sum = 0;
        let changed = self.sum_excel != sum;
        if changed {
            self.sum_excel = sum;
        }
        changed
    }

    /// 获取文档的类型以及同路径下的配置文件
    ///
    /// # Arguments
    ///
    /// * `global`:
    ///
    /// returns: Result<(), XError>
    ///
    /// # Examples
    ///
    /// ```
    /// use xcell_core;
    /// ```
    pub fn load_config(&mut self, global: &ProjectConfig) -> XResult<()> {
        let table = find_first_table(&self.path)?;
        self.headers = read_table_headers(&table)?;
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
            self.config = TableConfig::load_file(&config, Some(global))?;
        }
        Ok(())
    }
}

impl XCellTable {
    pub fn set_path(&mut self, path: &Path) -> XResult<()> {
        self.path = path.canonicalize()?;
        match self.try_set_name() {
            Some(_) => Ok(()),
            None => Err(XError::table_error(format!("配置表文件名不合法: {}", path.display()))),
        }
    }
    fn try_set_name(&mut self) -> Option<()> {
        let file = self.path.file_prefix()?.to_str()?;
        match file.is_empty() {
            true => return None,
            false => self.name = file.to_string(),
        }
        Some(())
    }
}
