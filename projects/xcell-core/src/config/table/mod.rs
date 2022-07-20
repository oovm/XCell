use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    pub typing: TypeMetaInfo,
    pub unity: TypeMetaInfo,
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
        TableConfig { typing: Default::default(), unity: project.unity.clone() }
    }
}
