use serde::{Deserialize, Serialize};

use super::*;

mod der;
mod ser;

/// XLua 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XluaCodegen {
    /// Whether to generate XLua code
    pub enable: bool,
    /// Output directory
    pub output: String,
    /// Generated manager name
    pub manager_name: String,
    /// Generated table name suffix
    pub suffix_table: String,
    /// Generated instance name
    pub instance_name: String,
}
