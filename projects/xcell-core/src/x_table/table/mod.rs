use super::*;

mod calamine_table;
mod display;
mod headers;
mod rows;

#[derive(Clone, Debug)]
pub struct CalamineTable {
    path: PathBuf,
    table: calamine::Range<DataType>,
    config: TableConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XTable {
    /// 表格的绝对路径
    pub path: PathBuf,
    /// 表格的名称, 同时也是生成的类名
    pub name: String,
    /// 表格的额外配置
    pub config: TableConfig,
    /// 表格中的有效数据
    pub data: XExportData,
    /// 枚举定义是否已链接
    pub enumeration_linked: bool,
    /// 产物是否已生成
    pub output_generated: bool,
}
