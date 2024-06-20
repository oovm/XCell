use serde::{Deserialize, Serialize};

use super::*;

pub mod json;
pub mod sql;
pub mod typescript;
pub mod xlua;

/// 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodegenConfig {
    pub json: json::JsonCodegen,
    pub sql: sql::SqlCodegen,
    pub typescript: typescript::TypeScriptCodegen,
    pub xlua: xlua::XluaCodegen,
}
