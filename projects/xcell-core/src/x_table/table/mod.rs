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
    pub fn load_file(path: PathBuf, global: &ProjectConfig) -> Validation<Self> {
        let mut errors = vec![];
        let mut xcell = Self::default();
        let fatal: XResult<()> = try {
            xcell.path = path.canonicalize()?;
            xcell.load_config(global)?;
        };
        if let Err(fatal) = fatal {
            return Failure { fatal, diagnostics: errors };
        }
        if xcell.check_sum_change() {
            match xcell.load_data() {
                Success { diagnostics, .. } => errors.extend(diagnostics),
                Failure { diagnostics, fatal } => {
                    errors.extend(diagnostics);
                    return Failure { fatal, diagnostics: errors };
                }
            }
        }
        Success { value: xcell, diagnostics: errors }
    }
    /// 强制重新加载表格中的数据
    pub fn load_data(&mut self) -> Validation<()> {
        match find_first_table(&self.path) {
            Ok(table) => read_table_data(&table, &self.header).map(|v| self.data = v),
            Err(e) => Failure { fatal: e, diagnostics: vec![] },
        }
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
        let sum = 0;
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
        self.header = read_table_headers(&table)?;
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
    /// 加载表格数据
    fn read_excel(&mut self) {}
}
