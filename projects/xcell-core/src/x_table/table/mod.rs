use crate::{
    utils::{find_first_table, read_table_headers},
    x_table::global_config::ProjectConfig,
};

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
    pub fn load_file(path: PathBuf, global: &ProjectConfig) -> XResult<Self> {
        let mut xcell = Self::default();
        xcell.path = path.canonicalize()?;
        let table = find_first_table(&xcell.path)?;
        xcell.headers = read_table_headers(&table)?;
        xcell.load_config(global)?;
        if xcell.check_sum_change() {}
        Ok(xcell)
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
    /// 获取同路径下的配置文件
    ///
    /// 要在 [`find_table_headers`] 之后执行, 以防类型被覆盖
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
    /// 加载表格数据
    fn read_excel(&mut self) {}
}
