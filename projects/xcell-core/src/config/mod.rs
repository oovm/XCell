use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fmt::Debug,
    fs::read_to_string,
    path::{Path, PathBuf},
    str::FromStr,
    sync::LazyLock,
};
use toml::{from_str, Value};

use xcell_errors::{Failure, Success, XError, XResult};

use crate::{
    utils::{split_file_name, split_namespace, walk_blob_set},
    XCellTable,
};

pub use self::{
    project::ProjectConfig,
    unity::{UnityCodegen, UNITY_CODEGEN_CONFIG},
};

mod der;
mod project;
mod table;
mod unity;

#[derive(Debug, Clone)]
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
        let input = workspace.as_ref();
        let root = input.canonicalize()?;
        if !root.is_dir() {
            return Err(XError::table_error(format!("{} 不是目录名", input.display())));
        }
        else {
            log::info!("工作目录: {}", root.display());
        }
        let config = ProjectConfig::new(&root);
        Ok(Self { root, config })
    }
    /// 首次加载目录
    pub async fn first_walk(&mut self) {
        let unity = UnityCodegen::default();
        for excel in walk_blob_set(&self.root, &self.config.glob).await.unwrap() {
            match XCellTable::load_file(&excel, &self.config) {
                Success { value, diagnostics } => {
                    for e in diagnostics {
                        log::error!("{}", e)
                    }
                    match unity.write_class(&value) {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("{}", e)
                        }
                    }
                }
                Failure { fatal, diagnostics } => {
                    for e in diagnostics {
                        log::error!("{}", e)
                    }
                    log::error!("{}", fatal)
                }
            }
        }
    }
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
