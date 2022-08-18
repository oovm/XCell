use super::*;

mod der;

#[derive(Debug, Clone, Default, Serialize)]
pub struct TableConfig {
    pub line: TableLineMode,
    pub typing: TypeMetaInfo,
    pub unity: UnityCodegen,
}

#[derive(Copy, Clone, Debug, Serialize)]
pub struct TableLineMode {
    /// 字段名写在表格的第几行, 默认第一行
    pub typing: usize,
    /// 类型写在表格的第几行, 默认第二行
    pub field: usize,
    /// 注释写在表格的第几行, 默认第三行
    pub helper: usize,
    /// 数据从表格第几行开始, 默认第四行
    pub data: usize,
}

impl TableConfig {
    /// 配置路径, 必须是 toml 格式
    pub fn load_file(path: Option<&Path>, global: Option<&ProjectConfig>) -> XResult<Self> {
        let basic = match global {
            Some(s) => TableConfig::from(s),
            None => Default::default(),
        };
        let _: Option<()> = try {
            let text = read_to_string(path?).ok()?;
            let table = text.parse::<Value>().ok()?;
            if let Ok(o) = table.deserialize_any(basic.clone()) {
                return Ok(o);
            }
        };
        Ok(basic)
    }
}

impl From<&ProjectConfig> for TableConfig {
    fn from(project: &ProjectConfig) -> Self {
        TableConfig { line: project.line, typing: project.typing.clone(), unity: project.unity.clone() }
    }
}
