use std::{
    fmt::{Debug, Formatter},
    fs::metadata,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{XError, XResult};
use xcell_types::for_3rd::{GlobSet, StreamExt, build_glob_set, file_watcher};

use crate::{
    LanguageManager, XClassTable, XDictTable, XEnumerateTable, XLanguageID, XLanguageTable, XListTable,
    utils::{get_relative, valid_file},
    validation::{ValidationManager, ValidationResult},
    x_table::{enumerate::DefineManager, table::CalamineTable},
};
use xcell_config::{PROJECT_CONFIG, ProjectConfig, TableConfig, TableLineMode, UnityBinaryConfig, UnityCodegen};
use xcell_plugin::{PluginManager, WorkspaceManager as PluginWorkspaceManager};

pub struct WorkspaceManager {
    pub config: ProjectConfig,
    pub glob_pattern: GlobSet,
    pub defines: DefineManager,
    pub languages: LanguageManager,
    pub file_modification_times: std::collections::HashMap<PathBuf, SystemTime>,
    pub validation_manager: ValidationManager,
    pub plugin_manager: PluginManager,
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
        };
        Ok(workspace)
    }
    /// 首次加载目录
    pub fn first_walk(&mut self) -> XResult<()> {
        let glob = build_glob_set(&self.config.include).result(|e| tracing::error!("{e}"))?;
        let entries = xcell_types::for_3rd::SyncWalkDir::new(&self.config.root);
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
                    let normed = get_relative(&self.config.root, file)?;
                    if glob.is_match(&normed) {
                        tracing::info!("首次加载: {}", normed.display());
                        self.load_file(&file);
                        // 记录文件修改时间
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
        self.write_unity()?;
        self.write_cocos()?;
        Ok(())
    }
    pub async fn watcher(&mut self) -> XResult<()> {
        let mut watcher = file_watcher(&self.config.root)?;
        loop {
            match watcher.next().await {
                Some(Ok(o)) => {
                    tracing::trace!("文件变更: {:?}", o);
                    // 处理文件修改事件
                    for path in o.paths {
                        // 直接检查路径是否为有效文件
                        if path.is_file() {
                            let normed = get_relative(&self.config.root, &path)?;
                            if self.glob_pattern.is_match(&normed) {
                                // 检查文件是否过期
                                if let Ok(meta) = metadata(&path) {
                                    if let Ok(mtime) = meta.modified() {
                                        if let Some(old_mtime) = self.file_modification_times.get(&path) {
                                            if mtime > *old_mtime {
                                                tracing::info!("文件过期: {}", normed.display());
                                                // 重新加载文件
                                                self.load_file(&path);
                                                // 更新修改时间
                                                self.file_modification_times.insert(path, mtime);
                                            }
                                        }
                                        else {
                                            // 新文件，加载并记录修改时间
                                            tracing::info!("新文件: {}", normed.display());
                                            self.load_file(&path);
                                            self.file_modification_times.insert(path, mtime);
                                        }
                                    }
                                }
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
}

impl WorkspaceManager {
    /// path 需要是绝对路径
    pub fn load_file(&mut self, file: &Path) {
        if let Err(e) = self.try_perform_file(file) {
            tracing::error!("{e}")
        }
    }
    pub fn try_perform_file(&mut self, file: &Path) -> XResult<()> {
        let table = crate::x_table::load_table(file, &self.config)?;

        // 执行数据验证
        let validation_result = self.validation_manager.validate(table.as_ref(), self);
        if validation_result.has_errors() {
            for error in validation_result.errors {
                tracing::error!("{}", error.with_path(file));
            }
        }

        let result = if let Ok(s) = XListTable::confirm(crate::x_table::table::ArcTableReader::new(table.clone())) {
            for error in s.perform(self) {
                tracing::error!("{}", error.with_path(file));
            }
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
            self.write_unity()?;
            self.write_cocos()?;
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

    pub fn write_unity(&self) -> XResult<()> {
        // 这里需要根据条件导出的逻辑修改实际的导出代码
        // 暂时保持原有逻辑，后续需要根据 should_export 方法来过滤需要导出的表格
        self.config.unity.write_binary()?;
        
        // 直接在 WorkspaceManager 中实现 Unity C# 代码生成
        use std::fs::File;
        use std::io::Write;
        use std::path::PathBuf;
        
        let unity = &self.config.unity;
        let root = &self.config.root;
        
        // 确保输出目录存在
        let output_dir = PathBuf::from(&unity.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // 为每个类表生成 C# 代码
        for table in self.defines.class.values() {
            let table_name = format!("{}{}", table.name, unity.suffix_table);
            let path = output_dir.join(table_name).with_extension("cs");
            
            // 创建目录
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            // 生成 C# 代码
            let mut file = File::create(path)?;
            writeln!(file, "// Unity generated file")?;
            writeln!(file, "")?;
            writeln!(file, "namespace {}", unity.namespace)?;
            writeln!(file, "{{")?;
            writeln!(file, "    public class {}", table_name)?;
            writeln!(file, "    {{")?;
            
            // 生成字段
            for item in &table.items {
                writeln!(file, "        public object {};", item.field)?;
            }
            
            writeln!(file, "    }}")?;
            writeln!(file, "}}")?;
        }

        // self.config.unity.write_manager(&self.collect_merged(), &self.config.root, &self.config.version)?;
        Ok(())
    }

    /// 生成 Cocos 代码和 JSON 数据
    pub fn write_cocos(&self) -> XResult<()> {
        if self.config.cocos.enable {
            use std::fs::File;
            use std::io::Write;
            use serde_json::json;
            
            let cocos = &self.config.cocos;
            
            // 确保输出目录存在
            if let Some(s) = cocos.cocos_typescript_path(&self.config.root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
            if cocos.json.enable {
                if let Some(s) = cocos.cocos_json_path(&self.config.root, "test")?.parent() {
                    std::fs::create_dir_all(s)?;
                }
            }
            
            // 为每个类表生成 TypeScript 代码
            for table in self.defines.class.values() {
                let table_name = format!("{}{}", table.name, cocos.suffix_table);
                let path = cocos.cocos_typescript_path(&self.config.root, &table_name)?;
                
                // 创建目录
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                
                // 生成 TypeScript 代码
                let mut file = File::create(path)?;
                writeln!(file, "// Cocos generated file")?;
                writeln!(file, "")?;
                writeln!(file, "namespace {}", cocos.namespace)?;
                writeln!(file, "{{")?;
                writeln!(file, "    export class {}", table_name)?;
                writeln!(file, "    {{")?;
                
                // 生成字段
                for item in &table.items {
                    writeln!(file, "        public {}: any;", item.field)?;
                }
                
                writeln!(file, "    }}")?;
                writeln!(file, "}}")?;
            }
            
            // 为每个类表生成 JSON 数据
            for table in self.defines.class.values() {
                let path = cocos.cocos_json_path(&self.config.root, &table.name)?;
                
                // 创建目录
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                
                // 生成 JSON 数据
                let mut file = File::create(path)?;
                let json_data = json!({
                    "name": table.name,
                    "items": table.items.iter().map(|item| {
                        let mut item_data = serde_json::Map::new();
                        item_data.insert("field".to_string(), json!(item.field));
                        item_data
                    }).collect::<Vec<_>>()
                });
                let json_str = serde_json::to_string_pretty(&json_data)
                    .map_err(|e| xcell_types::XError::runtime_error(format!("JSON serialization error: {}", e)))?;
                writeln!(file, "{}", json_str)?;
            }
        }
        Ok(())
    }
}
