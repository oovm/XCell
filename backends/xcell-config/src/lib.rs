//! XCell 配置模块
//!
//! 负责管理 XCell 的所有配置相关功能，包括项目配置、表格配置等。

pub mod cocos;
pub mod codegen;
pub mod merge_rules;
pub mod project;
pub mod table;
pub mod unity;

pub use self::{
    cocos::{CocosCodegen, CocosJsonConfig},
    codegen::{json::JsonCodegen, sql::SqlCodegen, typescript::TypeScriptCodegen, xlua::XluaCodegen},
    merge_rules::{MergeRules, MergeStep},
    project::{ExportCondition, ProjectConfig},
    table::{FieldConfig, TableConfig, TableLineMode},
    unity::{UnityBinaryConfig, UnityCodegen},
};

/// 默认的全局项目设置
pub const PROJECT_CONFIG: &str = include_str!("../ProjectConfig.toml");
