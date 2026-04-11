use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use xcell_core::{XError, XResult};

use super::*;

mod der;
mod ser;

/// Cocos 存储格式配置
#[derive(Debug, Clone)]
pub enum CocosStorage {
    /// JSON 存储配置
    Json(CocosJsonConfig),
}

impl Default for CocosStorage {
    fn default() -> Self {
        Self::Json(CocosJsonConfig::default())
    }
}

impl CocosStorage {
    pub fn output_path(&self) -> &str {
        match self {
            CocosStorage::Json(config) => config.output.as_ref(),
        }
    }
}

/// Cocos 代码生成配置
///
/// 用于配置 Cocos 平台的代码生成
#[derive(Debug, Clone)]
pub struct CocosCodegen {
    /// 是否要生成 cocos 代码
    pub enable: bool,
    /// cocos 的工作目录, 建议使用相对路径
    pub project: String,
    /// loader 的输出目录，以为 `project` 为根目录
    pub output: String,
    /// 生成的管理器的名称
    pub manager_name: String,
    /// 生成的表格名的后缀
    pub suffix_table: String,
    /// 生成的实例名称
    pub instance_name: String,
    /// 表数据路径前缀
    pub table_data_path: String,
    /// 存储格式配置
    pub storage: CocosStorage,
    /// 开发时用的储存格式
    pub storage_debug: Option<CocosStorage>,
}

impl Default for CocosCodegen {
    fn default() -> Self {
        Self {
            enable: true,
            project: "..".to_string(),
            output: "assets/scripts/dataTable/generated".to_string(),
            manager_name: "DataTableManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "DataTable".to_string(),
            table_data_path: "tables".to_string(),
            storage: CocosStorage::default(),
            storage_debug: None,
        }
    }
}

/// Cocos JSON 配置
///
/// 用于配置 JSON 数据生成
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CocosJsonConfig {
    /// 是否启用 JSON 生成
    pub enable: bool,
    /// data 的输出目录，以为 `project` 为根目录
    pub output: String,
}

impl CocosCodegen {
    /// 获取开发时存储配置
    pub fn get_development_storage(&self) -> &CocosStorage {
        self.storage_debug.as_ref().unwrap_or(&self.storage)
    }

    /// Cocos 项目文件夹
    pub fn cocos_path(&self, root: &Path) -> XResult<PathBuf> {
        let project = PathBuf::from(&self.project);
        let project = match project.is_absolute() {
            true => project,
            false => root.join(project),
        };
        Ok(project.canonicalize()?)
    }

    /// 生成 TypeScript 代码的文件夹
    pub fn cocos_typescript_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.cocos_path(root)?.join(&self.output);
        let path = dir.join(file_name).with_extension("ts");
        Ok(path)
    }

    /// 生成管理器路径
    pub fn cocos_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.cocos_typescript_path(root, &self.manager_name)
    }

    /// 生成 JSON 文件路径
    pub fn cocos_json_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let output = match &self.storage {
            CocosStorage::Json(config) => &config.output,
        };
        let dir = self.cocos_path(root)?.join(output);
        let path = dir.join(file_name).with_extension("json");
        Ok(path)
    }

    /// 生成 TypeScript 相对路径
    pub fn cocos_ts_relative(&self, file_name: &str) -> String {
        format!("{}/{}.ts", self.output, file_name)
    }

    /// 生成 JSON 相对路径
    pub fn cocos_json_relative(&self, file_name: &str) -> String {
        let output = match &self.storage {
            CocosStorage::Json(config) => &config.output,
        };
        format!("{}/{}.json", output, file_name)
    }

    /// 数据的路径，以项目路径为基准，**必须是**相对路径。
    ///
    /// 默认同加载器路径。
    pub fn data_path(&self, config: &Path) -> PathBuf {
        let data_path = self.storage.output_path();
        if data_path.is_empty() {
            return self.cocos_path(config).unwrap().join(&self.output);
        }
        self.cocos_path(config).unwrap().join(data_path)
    }

    /// 数据的路径，以项目路径为基准，**必须是**相对路径。
    ///
    /// 默认同数据路径。
    pub fn debug_data_path(&self, config: &Path) -> PathBuf {
        let debug_path = match self.storage_debug.as_ref() {
            None => "",
            Some(s) => s.output_path(),
        };
        if debug_path.is_empty() {
            return self.data_path(config);
        }
        self.cocos_path(config).unwrap().join(debug_path)
    }

}
