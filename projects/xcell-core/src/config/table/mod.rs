use crate::config::unity::UnityCodegen;

use super::*;

mod der;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    pub line: LineMode,
    pub typing: UnityCodegen,
    pub unity: UnityCodegen,
}

#[derive(Debug, Clone, Serialize)]
pub struct LineMode {
    /// 字段名写在表格的第几行, 默认第一行
    #[serde(alias = "type")]
    #[serde(default = "1")]
    pub typing: usize,
    /// 类型写在表格的第几行, 默认第二行
    #[serde(default = "2")]
    pub field: usize,
    /// 注释写在表格的第几行, 默认第三行
    #[serde(alias = "comment")]
    #[serde(default = "3")]
    pub helper: usize,
    /// 数据从表格第几行开始, 默认第四行
    #[serde(default = "4")]
    pub data: usize,
}

impl TableConfig {
    /// 配置路径, 必须是 toml 格式
    pub fn load_file(path: Option<&Path>, global: Option<&ProjectConfig>) -> XResult<Self> {
        let mut basic = match global {
            Some(s) => TableConfig::from(s),
            None => Default::default(),
        };
        match path {
            Some(path) => match read_to_string(path) {
                Ok(o) => {
                    log::info!("载入配置: {}", path.file_name().and_then(OsStr::to_str).unwrap_or(""));
                    let toml = o.parse::<Value>()?;
                    basic.read_toml(&toml);
                }
                Err(..) => {}
            },
            None => {}
        }

        Ok(basic)
    }
    fn read_toml(&mut self, root: &Value) -> Option<()> {
        let fields_table = root.get("field")?.as_table()?;
        for (field, desc) in fields_table {
            log::error!("{field}: {desc}")
        }
        None
    }
}

impl From<&ProjectConfig> for TableConfig {
    fn from(project: &ProjectConfig) -> Self {
        TableConfig { typing: Default::default(), unity: project.unity.clone(), lines: 0, line_field: 0, line_comment: 0 }
    }
}
