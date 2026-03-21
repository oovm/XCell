use serde::{Deserialize, Serialize};

use xcell_types::{XError, XResult};

use super::*;

mod der;
mod ser;

/// Unity 存储格式配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityStorage {
    /// 二进制存储配置
    #[serde(default)]
    pub binary: UnityBinaryConfig,
    /// JSON 存储配置
    #[serde(default)]
    pub json: UnityJsonConfig,
    /// XML 存储配置
    #[serde(default)]
    pub xml: UnityXmlConfig,
    /// Protobuf 存储配置
    #[serde(default)]
    pub protobuf: UnityProtobufConfig,
}

/// Unity 加载器配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityLoader {
    /// C# 加载器配置
    pub enable: bool,
    pub project: String,
    pub output: String,
    pub namespace: String,
    pub manager: String,
    pub suffix_table: String,
    pub suffix_element: String,
    pub support_clone: bool,
    pub legacy_using: bool,
    pub legacy_null_null: bool,
    /// XLua 加载器配置
    #[serde(default)]
    pub xlua: UnityXluaConfig,
}

/// Unity 代码生成配置
#[derive(Debug, Clone, Default, Serialize)]
pub struct UnityCodegen {
    /// 存储格式配置
    #[serde(default)]
    pub storage: UnityStorage,
    /// 加载器配置
    #[serde(default)]
    pub loader: UnityLoader,
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
