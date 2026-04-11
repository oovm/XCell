//! 表格配置模块
//!
//! 提供表格行模式配置和字段配置的定义。

use serde::{Deserialize, Serialize};

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

/// 表格行模式配置，定义各语义行在表格中的位置。
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TableLineMode {
    /// 字段名写在表格的第几行，默认第一行
    #[serde(default = "default_field_line")]
    pub field: usize,
    /// 类型写在表格的第几行，默认第二行
    #[serde(default = "default_type_line")]
    pub r#type: usize,
    /// 注释写在表格的第几行，默认第三行
    #[serde(default = "default_comment_line")]
    pub comment: usize,
    /// 数据从表格第几行开始，默认第四行
    #[serde(default = "default_data_line")]
    pub data: usize,
}

/// 默认字段行号（1-based）
fn default_field_line() -> usize {
    1
}

/// 默认类型行号（1-based）
fn default_type_line() -> usize {
    2
}

/// 默认注释行号（1-based）
fn default_comment_line() -> usize {
    3
}

/// 默认数据行号（1-based）
fn default_data_line() -> usize {
    4
}
