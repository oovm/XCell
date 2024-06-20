use serde::{Deserialize, Serialize};

use super::*;

mod der;
mod ser;

/// SQL 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlCodegen {
    /// Whether to generate SQL code
    pub enable: bool,
    /// Output directory
    pub output: String,
    /// Database name
    pub database: String,
}
