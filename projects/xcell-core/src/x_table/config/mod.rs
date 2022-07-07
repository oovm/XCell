use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use xcell_errors::XError;

use super::*;

pub use self::project::ProjectConfig;

mod der;
mod project;
mod table;
mod unity;

#[derive(Debug, Clone, Default)]
pub struct WorkspaceManager {
    pub root: PathBuf,
    pub config: ProjectConfig,
}

impl WorkspaceManager {
    /// 设置工作目录
    pub fn new<P>(workspace: P) -> XResult<Self>
    where
        P: AsRef<Path>,
    {
        let root = workspace.as_ref().canonicalize()?;
        Ok(Self { root, ..Default::default() })
    }
    /// 首次加载目录
    pub fn first_walk(&mut self) {}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    pub typing: TypeMetaInfo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    pub boolean: BooleanMetaInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct BooleanMetaInfo {
    pub r#true: BTreeSet<String>,
    pub r#false: BTreeSet<String>,
    pub default: bool,
}
