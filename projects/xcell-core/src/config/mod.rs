use std::{
    any::type_name,
    collections::BTreeMap,
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
use xcell_types::{default_deserialize, EnumerateDescription, TypeMetaInfo};

use crate::{
    config::unity::UnityCodegen,
    utils::{get_relative, valid_file},
    x_table::language::manager::LanguageManager,
    EnumerateManager, XEnumerateTable, XExportData, XLanguageTable, XTable,
};

pub use self::{
    project::ProjectConfig,
    table::{TableConfig, TableLineMode},
    unity::UnityBinaryConfig,
};

pub mod merge_rules;
mod project;
mod table;
pub mod unity;
use crate::x_table::table::CalamineTable;

/// 默认的全局项目设置
pub const PROJECT_CONFIG: &str = include_str!("ProjectConfig.toml");

pub struct WorkspaceManager {
    pub config: ProjectConfig,
    pub glob_pattern: GlobSet,
    pub file_mapping: BTreeMap<PathBuf, XTable>,
    pub enumerates: EnumerateManager,
    pub languages: LanguageManager,
}

default_deserialize![ProjectConfig, TableConfig, TableLineMode];

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
        Ok(Self {
            config,
            glob_pattern,
            file_mapping: Default::default(),
            enumerates: Default::default(),
            languages: Default::default(),
        })
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
                        self.load_file(&file)
                    }
                }
                None => break,
                _ => continue,
            }
        }
        self.link_enumerate();
        self.write_unity()?;
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

impl WorkspaceManager {
    /// path 需要是绝对路径
    pub fn load_file(&mut self, file: &Path) {
        if let Err(e) = self.try_perform_file(file) {
            log::error!("{e}")
        }
    }
    pub fn try_perform_file(&mut self, file: &Path) -> XResult<()> {
        let table = CalamineTable::load(file, &self.config)?;
        if let Ok(s) = XLanguageTable::confirm(&table) {
            return s.perform(&mut self);
        }
        if let Ok(s) = XEnumerateTable::confirm(&table) {
            return s.perform(&mut self);
        }
        Ok(())
    }
    pub fn write_unity(&self) -> XResult<()> {
        for table in self.file_mapping.values() {
            table.config.unity.ensure_path(&self.config.root)?;
            table.config.unity.write_class(table, &self.config.root)?;
            table.config.unity.write_binary(table, &self.config.root)?;
            table.config.unity.write_data_contract(table, &self.config.root)?;
        }
        self.config.unity.write_manager(&self.collect_merged(), &self.config.root, &self.config.version)?;
        Ok(())
    }
}
