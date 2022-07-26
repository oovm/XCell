use super::*;
use crate::config::table::LineMode;

#[derive(Debug)]
pub struct ProjectConfig {
    pub root: PathBuf,
    /// 当前版本号
    pub version: String,
    /// 包含的 excel 路径, 优先级最高
    pub include: String,
    /// 排除的 excel 模式, 优先级低于 include
    pub exclude: String,
    /// 行列排序模式
    pub line: LineMode,
    /// 类型解析模式
    pub typing: TypeMetaInfo,
    /// Unity 生成模式
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
            match v.read_config(&cfg) {
                Ok(_) => {
                    log::info!("加载项目配置成功, 当前配置\n{v:#?}");
                }
                Err(e) => {
                    log::error!("{e}");
                    log::info!("加载项目配置失败, 当前配置\n{v:#?}");
                }
            }
        }
        else {
            log::info!("XCell.toml 不存在, 使用内置项目配置\n{v:#?}");
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
