use toml::from_str;

use super::*;
use crate::{config::table::TableLineMode, MergeRules};
use xcell_types::TypeMetaInfo;

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
    /// 合表模式
    pub merge: MergeRules,
    /// Unity 生成模式
    pub unity: UnityCodegen,
}

mod der;
mod ser;

impl ProjectConfig {
    pub fn new(workspace: &Path) -> Self {
        log::info!("工作目录: {}", workspace.display());
        let cfg = workspace.join("XCell.toml");
        let success;
        let mut config = match Self::load_toml(&cfg) {
            Ok(o) => {
                success = "成功";
                o
            }
            Err(e) => {
                success = "失败";
                log::error!("{}", e.with_path(&cfg));
                Default::default()
            }
        };
        config.root = workspace.to_path_buf();
        log::info!("加载项目配置{success}, 当前配置\n{config:#?}");
        config
    }
    fn load_toml(file: &Path) -> XResult<Self> {
        if file.exists() {
            let text = read_to_string(file)?;
            Ok(from_str(&text)?)
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
