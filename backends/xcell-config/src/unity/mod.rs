use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::*;

mod der;
mod ser;

/// Unity 存储格式配置
#[derive(Debug, Clone)]
pub enum UnityStorage {
    /// 二进制存储配置
    Binary(UnityBinaryConfig),
    /// JSON 存储配置
    Json(UnityJsonConfig),
    /// XML 存储配置
    Xml(UnityXmlConfig),
    /// Protobuf 存储配置
    Protobuf(UnityProtobufConfig),
}

impl Default for UnityStorage {
    fn default() -> Self {
        Self::Binary(UnityBinaryConfig::default())
    }
}

/// Unity 代码生成配置
#[derive(Debug, Clone, Default)]
pub struct UnityCodegen {
    /// C# 加载器配置
    pub enable: bool,
    /// cocos 的工作目录, 相对于配置文件
    pub project: String,
    /// loader 输出目录，以 `project` 为根目录
    pub output: String,
    pub namespace: String,
    pub manager: String,
    pub suffix_table: String,
    pub suffix_element: String,
    pub support_clone: bool,
    pub legacy_using: bool,
    pub legacy_null_null: bool,
    /// XLua 加载器配置
    pub xlua: UnityXluaConfig,
    /// 存储格式配置
    pub storage: UnityStorage,
    /// 开发时用的储存格式
    pub storage_debug: Option<UnityStorage>,
}

/// Unity XLua 配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityXluaConfig {
    pub enable: bool,
}

/// Unity XML 配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityXmlConfig {
    pub enable: bool,
    pub output: String,
}

/// Unity JSON 配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityJsonConfig {
    pub enable: bool,
    pub output: String,
}

/// Unity Protobuf 配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityProtobufConfig {
    pub enable: bool,
    pub output: String,
}

impl UnityStorage {
    pub fn output_path(&self) -> &str {
        match self {
            UnityStorage::Binary(x) => x.output.as_ref(),
            UnityStorage::Json(x) => x.output.as_ref(),
            UnityStorage::Xml(x) => x.output.as_ref(),
            UnityStorage::Protobuf(x) => x.output.as_ref(),
        }
    }
}

/// Unity 代码生成配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityCodegenConfig {
    pub class: bool,
    pub dictionary: bool,
    pub enumerate: bool,
    pub language: bool,
    pub manager: bool,
    pub binary: bool,
    pub csharp: bool,
}

/// Unity 二进制配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityBinaryConfig {
    pub enable: bool,
    pub output: String,
}

impl UnityCodegen {
    /// 项目路径，建议使用相对路径。
    pub fn project_path(&self, config: &Path) -> PathBuf {
        let project = Path::new(&self.project);
        if project.is_absolute() {
            return project.to_path_buf();
        }
        config.join(project)
    }

    /// 加载器的路径，以项目路径为基准，**必须是**相对路径。
    ///
    /// 默认同项目路径。
    pub fn loader_path(&self, config: &Path) -> PathBuf {
        self.project_path(config).join(&self.output)
    }

    /// 数据的路径，以项目路径为基准，**必须是**相对路径。
    ///
    /// 默认同加载器路径。
    pub fn data_path(&self, config: &Path) -> PathBuf {
        let data_path = self.storage.output_path();
        if data_path.is_empty() {
            return self.loader_path(config);
        }
        self.project_path(config).join(data_path)
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
        self.project_path(config).join(debug_path)
    }
}
