use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use toml::Value;

use xcell_errors::{
    for_3rd::{
        build_glob_set, file_watcher, read_map_next_extra, read_map_next_key_lowercase, read_map_next_value, GlobSet,
        StreamExt, WalkDir,
    },
    XError, XResult,
};
use xcell_types::{default_deserialize, BooleanDescription};

use crate::{
    utils::{get_relative, valid_file},
    XCellTable,
};

pub use self::{
    project::ProjectConfig,
    table::{TableConfig, TableLineMode},
    typing::TypeMetaInfo,
    unity::{UnityBinaryConfig, UnityCodegen},
};

mod project;
mod table;
mod typing;
mod unity;

pub struct WorkspaceManager {
    pub config: ProjectConfig,
    pub glob_pattern: GlobSet,
}

default_deserialize![ProjectConfig, UnityCodegen, UnityBinaryConfig, TableConfig, TypeMetaInfo, TableLineMode];

impl Debug for WorkspaceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkspaceManager")
            .field("workspace", &self.config.root.display())
            .field("config", &self.config)
            .finish()
    }
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
        let config = ProjectConfig::new(&root);
        let glob_pattern = build_glob_set(&config.include).unwrap();
        Ok(Self { config, glob_pattern })
    }
    /// 首次加载目录
    pub async fn first_walk(&mut self) -> XResult<()> {
        let glob = build_glob_set(&self.config.include).result(|e| log::error!("{e}"))?;
        let mut entries = WalkDir::new(&self.config.root);
        loop {
            match entries.next().await {
                Some(Ok(o)) if valid_file(&o) => {
                    let file = o.path();
                    let normed = self.get_relative(&file)?;
                    if glob.is_match(&normed) {
                        log::info!("首次加载: {}", normed.display());
                        match XCellTable::load_file(&file, &self.config) {
                            Ok(value) => match value.config.unity.write_class(&value) {
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
        let mut watcher = file_watcher(&self.config.root)?;
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

/// 默认的全局项目设置
pub const PROJECT_CONFIG: &str = include_str!("ProjectConfig.toml");
