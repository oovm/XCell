use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    pub typing: TypeMetaInfo,
    pub unity: UnityCodegen,
}

impl TableConfig {
    pub fn load_file(path: &Path, global: Option<&ProjectConfig>) -> XResult<Self> {
        let mut basic = match global {
            Some(s) => TableConfig::from(s),
            None => Default::default(),
        };
        match read_to_string(path) {
            Ok(o) => {
                log::info!("加载设置: {}", path.display());
                let toml = o.parse::<Value>()?;
                basic.read_toml(&toml);
            }
            Err(_) => {}
        }

        Ok(basic)
    }
    fn read_toml(&mut self, root: &Value) -> Option<()> {
        let fields_table = root.get("field")?.as_table()?;
        for (field, desc) in fields_table {
            println!("{field}: {desc}")
        }
        None
    }
}

impl From<&ProjectConfig> for TableConfig {
    fn from(project: &ProjectConfig) -> Self {
        TableConfig { typing: Default::default(), unity: project.unity }
    }
}
