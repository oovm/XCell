use serde::{Deserialize, Serialize};

use super::*;

mod der;
mod ser;

/// TypeScript 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptCodegen {
    /// Whether to generate TypeScript code
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
