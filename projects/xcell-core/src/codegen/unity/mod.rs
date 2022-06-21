use super::*;

mod config;
mod write_cs;

pub const UNITY_CODEGEN_CONFIG: &str = include_str!("config.toml");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnityCodegen {
    pub namespace: String,
    pub folder_binary: String,
    pub manager_name: String,
    pub suffix_table: String,
    pub suffix_element: String,
    pub support_binary: bool,
    pub support_clone: bool,
    pub legacy_using: bool,
}
