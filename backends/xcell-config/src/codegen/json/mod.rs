use serde::{Deserialize, Serialize};

use super::*;

mod der;
mod ser;

/// JSON 代码生成配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonCodegen {
    /// Whether to generate JSON code
    pub enable: bool,
    /// Output directory
    pub output: String,
}
