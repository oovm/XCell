use std::path::Path;

use serde::{Deserialize, Deserializer, Serialize};
use toml;

use xcell_core::{TypeMetaInfo, XResult};

use super::*;
use crate::{project::ProjectConfig, unity::UnityCodegen};

mod der;

/// 字段配置信息结构体，用于在 TOML 配置文件中定义字段的元信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldConfig {
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub r#type: String,
    /// 字段注释
    pub comment: Option<String>,
    /// 字段默认值
    pub default: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    pub line: TableLineMode,
    pub typing: TypeMetaInfo,
    /// 字段配置信息列表
    pub fields: Vec<FieldConfig>,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TableLineMode {
    /// 字段名写在表格的第几行, 默认第一行
    pub field: usize,
    /// 类型写在表格的第几行, 默认第二行
    pub r#type: usize,
    /// 注释写在表格的第几行, 默认第三行
    pub comment: usize,
    /// 数据从表格第几行开始, 默认第四行
    pub data: usize,
}

impl TableConfig {
    /// 配置路径, 必须是 toml 格式
    pub fn load_file(path: Option<&Path>, global: Option<&ProjectConfig>) -> XResult<Self> {
        let basic = match global {
            Some(s) => TableConfig::from(s),
            None => Default::default(),
        };
        if let Some(path) = path {
            if let Ok(text) = std::fs::read_to_string(path) {
                if let Ok(config) = toml::from_str(&text) {
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
