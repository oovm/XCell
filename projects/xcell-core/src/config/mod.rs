pub use self::{
    project::ProjectConfig,
    unity::{UnityCodegen, UNITY_CODEGEN_CONFIG},
};
use crate::utils::{split_file_name, split_namespace};
use log::trace;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs::read_to_string,
    path::{Path, PathBuf},
    str::FromStr,
    sync::LazyLock,
};
use toml::{from_str, Value};
use xcell_errors::{XError, XResult};
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
