use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub version: String,
    pub glob: String,
    pub unity: UnityCodegen,
}

/// 默认的全局项目设置
pub const PROJECT_CONFIG: &str = include_str!("ProjectConfig.toml");

static DEFAULT_CONFIG: LazyLock<ProjectConfig> = LazyLock::new(|| {
    let mut empty = ProjectConfig { version: "".to_string(), glob: Default::default(), unity: Default::default() };
    let root = from_str::<Value>(PROJECT_CONFIG).unwrap();
    empty.load_value(&root);
    log::trace!("初始化 PROJECT_CONFIG\n{empty:#?}");
    empty
});

impl Default for ProjectConfig {
    fn default() -> Self {
        DEFAULT_CONFIG.clone()
    }
}

impl ProjectConfig {
    pub fn new(workspace: &Path) -> Self {
        let mut v = ProjectConfig::default();

        let cfg = workspace.join("XCell.toml");
        if cfg.exists() {
            if let Err(e) = v.read_config(&cfg) {
                log::error!("加载项目配置失败: {e}");
            }
        }
        else {
            log::info!("XCell.toml 不存在, 使用内置项目设置");
        }
        v
    }
    fn read_config(&mut self, config: &Path) -> XResult<()> {
        let str = read_to_string(config)?;
        let root = from_str::<Value>(&str)?;
        Ok(self.load_value(&root))
    }
    fn load_value(&mut self, root: &Value) {
        let _: Option<()> = try { self.version = root.get("version")?.as_str()?.to_string() };
        let _: Option<()> = try { self.glob = root.get("glob")?.as_str()?.trim().to_string() };
    }
}
