use std::{
    fmt::{Debug, Formatter},
    fs::metadata,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{XError, XResult};
use xcell_core::for_3rd::{GlobSet, StreamExt, build_glob_set, file_watcher};

use crate::{
    LanguageManager, XClassData, XClassTable, XDictData, XDictTable, XEnumerateData, XEnumerateTable, XLanguageID, XLanguageTable, XListData, XListTable,
    utils::{get_relative, valid_file},
    validation::{ValidationManager, ValidationResult},
    x_table::{enumerate::DefineManager, table::CalamineTable},
};
use xcell_config::{PROJECT_CONFIG, ProjectConfig, TableConfig, TableLineMode, UnityBinaryConfig, UnityCodegen};
use xcell_plugin::{PluginManager, WorkspaceManager as PluginWorkspaceManager};

/// 工作空间管理器，负责管理配置表文件的加载、监控和导出
pub struct WorkspaceManager {
    /// 项目配置
    pub config: ProjectConfig,
    /// 文件匹配模式
    pub glob_pattern: GlobSet,
    /// 枚举定义管理器
    pub defines: DefineManager,
    /// 语言管理器
    pub languages: LanguageManager,
    /// 文件修改时间记录
    pub file_modification_times: std::collections::HashMap<PathBuf, SystemTime>,
    /// 验证管理器
    pub validation_manager: ValidationManager,
    /// 插件管理器
    pub plugin_manager: PluginManager,
    /// 文件变更回调
    pub on_file_changed: Option<std::sync::Arc<dyn Fn(&Self) + Send + Sync>>,
}

/// 工作空间状态信息
pub struct WorkspaceStatus {
    /// 列表表数量
    pub list_count: usize,
    /// 字典表数量
    pub dict_count: usize,
    /// 类表数量
    pub class_count: usize,
    /// 枚举表数量
    pub enumerate_count: usize,
    /// 语言数据数量
    pub language_count: usize,
    /// 已加载文件数量
    pub file_count: usize,
}

impl Debug for WorkspaceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkspaceManager")
            .field("workspace", &self.config.root.display())
            .field("config", &self.config)
            .finish()
    }
}

impl PluginWorkspaceManager for WorkspaceManager {
    fn get_config(&self) -> &dyn std::any::Any {
        &self.config
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
        let mut workspace = Self {
            config,
            glob_pattern,
            defines: Default::default(),
            languages: Default::default(),
            file_modification_times: Default::default(),
            validation_manager: Default::default(),
            plugin_manager: Default::default(),
            on_file_changed: None,
        };
        Ok(workspace)
    }
    /// 首次加载目录
    pub fn first_walk(&mut self, filter: Option<&str>) -> XResult<()> {
        let glob = build_glob_set(&self.config.include).result(|e| tracing::error!("{e}"))?;
        let filter_glob = filter.and_then(|f| build_glob_set(f).result(|e| tracing::error!("{e}")).ok());
        // 使用 project 字段作为文件遍历的根目录
        let project_path = self.config.root.join(&self.config.project);
        let entries = xcell_core::for_3rd::SyncWalkDir::new(&project_path);
        for entry in entries {
            match entry {
                Ok(o) => {
                    let path = o.path();
                    if !path.is_file() {
                        continue;
                    }
                    let file_name = path.file_name().unwrap_or_default();
                    let file_name = file_name.to_string_lossy();
                    if file_name.starts_with("~") {
                        continue;
                    }
                    let file = o.path();
                    let normed = get_relative(&project_path, file)?;
                    if glob.is_match(&normed) {
                        if let Some(ref fg) = filter_glob {
                            if !fg.is_match(&normed) {
                                continue;
                            }
                        }
                        tracing::info!("首次加载: {}", normed.display());
                        self.load_file(&file);
                        if let Ok(meta) = metadata(file) {
                            if let Ok(mtime) = meta.modified() {
                                self.file_modification_times.insert(file.to_path_buf(), mtime);
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
        self.link_enumerate();
        Ok(())
    }
    /// 启动文件监控，支持防抖和优雅退出
    pub async fn watcher(&mut self) -> XResult<()> {
        use tokio::time::{sleep, Duration};

        let mut watcher = file_watcher(&self.config.root)?;
        let debounce_delay = Duration::from_millis(500);

        loop {
            tokio::select! {
                event = watcher.next() => {
                    match event {
                        Some(Ok(o)) => {
                            tracing::trace!("文件变更: {:?}", o);
                            sleep(debounce_delay).await;

                            for path in o.paths {
                                if !path.exists() {
                                    if self.file_modification_times.remove(&path).is_some() {
                                        tracing::info!("文件已删除: {}", path.display());
                                    }
                                    continue;
                                }
                                if path.is_file() {
                                    let normed = get_relative(&self.config.root, &path)?;
                                    if self.glob_pattern.is_match(&normed) {
                                        if let Ok(meta) = metadata(&path) {
                                            if let Ok(mtime) = meta.modified() {
                                                if let Some(old_mtime) = self.file_modification_times.get(&path) {
                                                    if mtime > *old_mtime {
                                                        tracing::info!("文件已更新: {}", normed.display());
                                                        self.load_file(&path);
                                                        self.file_modification_times.insert(path, mtime);
                                                    }
                                                } else {
                                                    tracing::info!("新文件: {}", normed.display());
                                                    self.load_file(&path);
                                                    self.file_modification_times.insert(path, mtime);
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if let Some(ref callback) = self.on_file_changed {
                                callback(self);
                            }
                        }
                        None => break,
                        _ => continue,
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("收到退出信号，停止文件监控");
                    break;
                }
            }
        }
        Ok(())
    }
    /// 获取工作空间状态信息
    pub fn status(&self) -> WorkspaceStatus {
        WorkspaceStatus {
            list_count: self.defines.list.len(),
            dict_count: self.defines.dict.len(),
            class_count: self.defines.class.len(),
            enumerate_count: self.defines.enumerate.len(),
            language_count: self.languages.store.len(),
            file_count: self.file_modification_times.len(),
        }
    }
    /// 获取工作空间摘要信息
    pub fn summary(&self) -> String {
        let status = self.status();
        format!(
            "工作空间摘要:\n  根目录: {}\n  列表表: {} 个\n  字典表: {} 个\n  类表: {} 个\n  枚举表: {} 个\n  语言数据: {} 个\n  已加载文件: {} 个",
            self.config.root.display(),
            status.list_count,
            status.dict_count,
            status.class_count,
            status.enumerate_count,
            status.language_count,
            status.file_count
        )
    }
}

impl WorkspaceManager {
    /// path 需要是绝对路径
    pub fn load_file(&mut self, file: &Path) {
        if let Err(e) = self.try_perform_file(file) {
            tracing::error!("{e}")
        }
    }
    /// 尝试处理单个文件，解析并加载表格数据
    pub fn try_perform_file(&mut self, file: &Path) -> XResult<()> {
        tracing::debug!("处理文件: {}", file.display());
        let table = crate::x_table::load_table(file, &self.config)?;
        tracing::debug!("表格加载成功");

        // 执行数据验证
        let validation_result = self.validation_manager.validate(table.as_ref(), self);
        if validation_result.has_errors() {
            for error in validation_result.errors {
                tracing::error!("{}", error.with_path(file));
            }
        }

        // 尝试解析为 XListTable
        tracing::debug!("尝试解析为 XListTable");
        let result = if let Ok(s) = XListTable::confirm(crate::x_table::table::ArcTableReader::new(table.clone())) {
            tracing::debug!("XListTable::confirm 成功, 调用 perform");
            for error in s.perform(self) {
                tracing::error!("{}", error.with_path(file));
            }
            tracing::debug!("XListTable::perform 完成, 列表数量 = {}", self.defines.list.len());
            Ok(())
        }
        else if let Ok(s) = XDictTable::confirm(crate::x_table::table::ArcTableReader::new(table.clone())) {
            for error in s.perform(self) {
                tracing::error!("{}", error.with_path(file));
            }
            Ok(())
        }
        else if let Ok(s) = XEnumerateTable::confirm(crate::x_table::table::ArcTableReader::new(table.clone())) {
            for error in s.perform(self) {
                tracing::error!("{}", error.with_path(file));
            }
            self.link_enumerate();
            Ok(())
        }
        else if let Ok(s) = XClassTable::confirm(crate::x_table::table::ArcTableReader::new(table.clone())) {
            s.perform(self)
        }
        else if let Ok(s) = XLanguageTable::confirm(crate::x_table::table::ArcTableReader::new(table.clone())) {
            for error in s.perform(self) {
                tracing::error!("{}", error.with_path(file));
            }
            Ok(())
        }
        else if let Ok(s) = XLanguageID::confirm(crate::x_table::table::ArcTableReader::new(table.clone())) {
            for error in s.perform(self) {
                tracing::error!("{}", error.with_path(file));
            }
            Ok(())
        }
        else {
            Err(XError::table_error(format!("{} 不是有效的表格类型", table.as_ref().get_header(0).field_name)).with_path(file))
        };

        // 执行导出
        if result.is_ok() {
            // 代码生成现在由 xcell 可执行文件中的 xcell-generator 模块处理
            // self.write_unity()?;
            // self.write_cocos()?;
        }

        result
    }
    /// 检查表格是否符合导出条件
    pub fn should_export(&self, table_name: &str, target: &str) -> bool {
        for condition in &self.config.export_conditions {
            // 使用简单的字符串匹配，后续可以使用 globset 进行更复杂的匹配
            if condition.table_pattern == "*" || table_name.contains(&condition.table_pattern) {
                return condition.target == "both" || condition.target == target;
            }
        }
        true // 默认导出
    }

    /// 生成 Unity 代码和资源文件
    pub fn write_unity(&self) -> XResult<()> {
        // 代码生成现在由 xcell 可执行文件中的 xcell-generator 模块处理
        Ok(())
    }

    /// 生成 Cocos 代码和 JSON 数据
    pub fn write_cocos(&self) -> XResult<()> {
        // 代码生成现在由 xcell 可执行文件中的 xcell-generator 模块处理
        Ok(())
    }
}