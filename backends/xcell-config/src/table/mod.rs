use std::path::Path;

use serde::{Deserialize, Deserializer, Serialize};
use oak_toml::from_str;

use xcell_core::{FieldConfig, TableLineMode, TypeMetaInfo, XResult};

use super::*;
use crate::project::ProjectConfig;

mod der;

/// 表格配置结构体，包含行模式、类型元信息和字段配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    /// 行模式配置
    pub line: TableLineMode,
    /// 类型元信息
    pub typing: TypeMetaInfo,
    /// 字段配置信息列表
    pub fields: Vec<FieldConfig>,
}

impl TableConfig {
    /// 配置路径，必须是 toml 格式
    pub fn load_file(path: Option<&Path>, global: Option<&ProjectConfig>) -> XResult<Self> {
        let basic = match global {
            Some(s) => TableConfig::from(s),
            None => Default::default(),
        };
        if let Some(path) = path {
            if let Ok(text) = std::fs::read_to_string(path) {
                if let Ok(config) = from_str(&text) {
                    return Ok(config);
                }
            }
        }
        Ok(basic)
    }
}

impl From<&ProjectConfig> for TableConfig {
    fn from(project: &ProjectConfig) -> Self {
        TableConfig { line: project.line, typing: project.typing.clone(), fields: Vec::new() }
    }
}
