use super::*;

#[derive(Debug)]
pub struct ProjectConfig {
    pub root: PathBuf,
    pub version: String,
    pub include: String,
    pub exclude: String,
    pub typing: TypeMetaInfo,
    pub unity: UnityCodegen,
}

mod der;
mod ser;

impl ProjectConfig {
    pub fn new(workspace: PathBuf) -> Self {
        log::info!("工作目录: {}", workspace.display());
        let mut v = ProjectConfig { root: workspace, ..Default::default() };
        let cfg = v.root.join("XCell.toml");
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
        self.load_value(&root);
        Ok(())
    }
    fn load_value(&mut self, root: &Value) {
        let _: Option<()> = try { self.version = root.get("version")?.as_str()?.to_string() };
        let _: Option<()> = try { self.include = root.get("glob")?.as_str()?.trim().to_string() };
    }
}

impl ProjectConfig {
    pub fn get_relative(&self, file: &Path) -> XResult<PathBuf> {
        get_relative(file, &self.root)
    }
}

impl WorkspaceManager {
    pub fn get_relative(&self, file: &Path) -> XResult<PathBuf> {
        self.config.get_relative(file)
    }
}
