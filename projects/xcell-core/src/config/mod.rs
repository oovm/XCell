use std::{
    collections::BTreeSet,
    fmt::Debug,
    fs::read_to_string,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use serde::{Deserialize, Serialize};
use toml::{from_str, Value};

use xcell_errors::{
    for_3rd::{file_watcher, StreamExt, WalkDir},
    XError, XResult,
};

pub use self::{
    project::ProjectConfig,
    table::TableConfig,
    unity::{UnityCodegen, UNITY_CODEGEN_CONFIG},
};
use crate::{
    utils::{make_relative, split_file_name, split_namespace, valid_file},
    XCellTable,
};
use xcell_errors::for_3rd::build_glob_set;
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
    pub async fn first_walk(&mut self) -> XResult<()> {
        let unity = UnityCodegen::default();
        let glob = build_glob_set(&self.config.glob).result(|e| log::error!("{e}"))?;
        let mut entries = WalkDir::new(&self.root);
        loop {
            match entries.next().await {
                Some(Ok(o)) if valid_file(&o) => {
                    let file = o.path();
                    let normed = make_relative(&file, &self.root)?;
                    if glob.is_match(&normed) {
                        log::info!("首次加载: {}", normed.display());
                        match XCellTable::load_file(&file, &self.config) {
                            Ok(value) => match unity.write_class(&value) {
                                Ok(_) => {}
                                Err(e) => {
                                    log::error!("{}", e)
                                }
                            },
                            Err(e) => {
                                log::error!("{}", e)
                            }
                        }
                    }
                }
                None => break,
                _ => continue,
            }
        }
        Ok(())
    }
    pub async fn watcher(&mut self) -> XResult<()> {
        let mut watcher = file_watcher(&self.root)?;
        loop {
            match watcher.next().await {
                Some(Ok(o)) => {
                    log::trace!("文件变更: {:?}", o);
                }
                None => break,
                _ => continue,
            }
        }
        Ok(())
    }
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
