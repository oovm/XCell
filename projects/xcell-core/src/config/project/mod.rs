use super::*;
use crate::config::table::TableLineMode;
use toml::from_str;

#[derive(Clone, Debug)]
pub struct ProjectConfig {
    pub root: PathBuf,
    /// 当前版本号
    pub version: String,
    /// 包含的 excel 路径, 优先级最高
    pub include: String,
    /// 排除的 excel 模式, 优先级低于 include
    pub exclude: String,
    /// 行列排序模式
    pub line: TableLineMode,
    /// 类型解析模式
    pub typing: TypeMetaInfo,
    /// Unity 生成模式
    pub unity: UnityCodegen,
}

mod der;
mod ser;

impl ProjectConfig {
    pub fn new(workspace: &Path) -> Self {
        log::info!("工作目录: {}", workspace.display());
        let cfg = workspace.join("XCell.toml");
        let mut out = match Self::load_toml(&cfg) {
            Ok(v) => {
                log::info!("加载项目配置成功, 当前配置\n{v:#?}");
                v
            }
            Err(e) => {
                log::error!("{}", e.with_path(&cfg));
                let v = Default::default();
                log::info!("加载项目配置失败, 当前配置\n{v:#?}");
                v
            }
        };
        out.root = workspace.to_path_buf();
        out
    }
    fn load_toml(config: &Path) -> XResult<Self> {
        if config.exists() {
            let text = read_to_string(config)?;
            let out = from_str(&text)?;
            Ok(out)
        }
        else {
            Err(XError::table_error("XCell.toml 不存在"))
        }
    }
}

impl WorkspaceManager {
    pub fn get_relative(&self, file: &Path) -> XResult<PathBuf> {
        get_relative(file, &self.config.root)
    }
}
