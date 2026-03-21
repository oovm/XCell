use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use xcell_types::XResult;

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
    /// 获取开发时存储配置
    pub fn get_development_storage(&self) -> &UnityStorage {
        self.storage_debug.as_ref().unwrap_or(&self.storage)
    }
    pub fn project_path(&self, config: &Path) -> PathBuf {
        let project = Path::new(&self.project);
        if project.is_absolute() {
            return project.to_path_buf();
        }
        config.join(project)
    }

    pub fn loader_path(&self, config: &Path) -> PathBuf {
        self.project_path(config).join(&self.output)
    }

    pub fn production_data_path(&self, config: &Path) -> PathBuf {
        self.project_path(config).join(&self.output)
    }

    pub fn development_data_path(&self, config: &Path) -> PathBuf {
        self.project_path(config).join(&self.output)
    }
    
    /// 写入二进制数据
    pub fn write_binary(&self) -> XResult<()> {
        Ok(())
    }

    /// 写入 C# 代码
    pub fn write_csharp(&self) -> XResult<()> {
        Ok(())
    }

    /// 写入管理器
    pub fn write_manager(&self, data: &dyn std::any::Any, root: &std::path::Path, version: &str) -> XResult<()> {
        Ok(())
    }
}
