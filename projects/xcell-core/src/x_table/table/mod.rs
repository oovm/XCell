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
    pub fn load_file(excel: &Path, global: &ProjectConfig) -> XResult<Self> {
        let path = excel.canonicalize()?;

        let mut xcell = Self {
            path: excel.canonicalize()?,
            name: "".to_string(),
            data: Default::default(),
            config: Default::default(),
            sum_excel: 0,
            sum_config: 0,
        };
        // excel.metadata()?.modified()?;
        xcell.try_set_name()?;
        xcell.try_load_header(global)?;
        xcell.try_load_config(global)?;

        if xcell.check_sum_change() {
            match xcell.reload_data() {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{e}")
                }
            }
        }
        Ok(xcell)
    }
    fn try_set_name(&mut self) -> XResult<()> {
        let _: Option<()> = try {
            let file = self.path.file_prefix()?.to_str()?;
            if !file.is_empty() {
                self.name = file.to_string();
                return Ok(());
            }
        };
        Err(XError::table_error(format!("配置表文件名不合法: {}", self.path.display())))
    }
    fn try_load_header(&mut self, global: &ProjectConfig) -> XResult<()> {
        let err: XResult<()> = try {
            let table = find_first_table(&self.path)?;
            self.data.read_table_headers(&table, global);
        };
        err.map_err(|e| e.with_path(&self.path))
    }
    fn try_load_config(&mut self, global: &ProjectConfig) -> XResult<()> {
        let file = self.path.with_extension("toml");
        let file = if file.exists() { Some(file.as_path()) } else { None };
        self.config = TableConfig::load_file(file, Some(global))?;
        Ok(())
    }
    /// 强制重新加载表格中的数据
    pub fn reload_data(&mut self) -> XResult<()> {
        let table = find_first_table(&self.path)?;
        self.data.read_table_data(&table, &self.path);
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
}

impl XCellTable {
    pub fn on_config_change(&mut self, global: &ProjectConfig) -> XResult<()> {
        self.try_load_header(global)?;
        self.try_load_config(global)?;
        self.on_excel_change()?;
        Ok(())
    }
    pub fn on_excel_change(&mut self) -> XResult<()> {
        self.reload_data()?;
        Ok(())
    }
}
