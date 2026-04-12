use std::path::Path;

use serde::{Deserialize, Serialize};
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

/// 表格配置部分覆盖结构体，用于从 TOML 文件中读取部分配置并合并到基础配置。
///
/// 所有字段均为 `Option` 类型，缺失的字段不会覆盖基础配置中的对应值。
#[derive(Debug, Clone, Default, Deserialize)]
struct TableConfigPartial {
    /// 行模式配置，若 TOML 中未指定则为 None
    #[serde(default)]
    line: Option<TableLineMode>,
    /// 类型元信息，若 TOML 中未指定则为 None
    #[serde(default)]
    typing: Option<TypeMetaInfo>,
    /// 字段配置信息列表，若 TOML 中未指定则为 None
    #[serde(default)]
    fields: Option<Vec<FieldConfig>>,
}

impl TableConfig {
    /// 从文件加载表格配置，支持全局配置作为基础。
    ///
    /// 当 `path` 为 `None` 时，返回基于 `global` 的默认配置。
    /// 当 `path` 为 `Some` 时，读取 TOML 文件并解析为部分配置，
    /// 然后以 `global` 配置为基础，用文件中的非空字段覆盖。
    ///
    /// # Parameters
    /// - `path`: 表格配置文件路径，如果为 None 则返回基于全局配置的默认值
    /// - `global`: 全局项目配置，如果为 Some 则作为基础配置
    ///
    /// # Returns
    /// 加载并合并后的表格配置
    pub fn load_file(path: Option<&Path>, global: Option<&ProjectConfig>) -> XResult<Self> {
        let basic = match global {
            Some(s) => TableConfig::from(s),
            None => Default::default(),
        };
        if let Some(path) = path {
            if let Ok(text) = std::fs::read_to_string(path) {
                if let Ok(partial) = from_str::<TableConfigPartial>(&text) {
                    return Ok(basic.apply_partial(partial));
                }
            }
        }
        Ok(basic)
    }

    /// 将部分配置应用到当前配置上，非 None 的字段会覆盖当前值。
    fn apply_partial(self, partial: TableConfigPartial) -> Self {
        TableConfig {
            line: partial.line.unwrap_or(self.line),
            typing: partial.typing.unwrap_or(self.typing),
            fields: partial.fields.unwrap_or(self.fields),
        }
    }
}

impl From<&ProjectConfig> for TableConfig {
    fn from(project: &ProjectConfig) -> Self {
        TableConfig { line: project.line, typing: project.typing.clone(), fields: Vec::new() }
    }
}
