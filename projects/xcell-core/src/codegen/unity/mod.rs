use super::*;

mod write_cs;
mod config;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnityCodegen {
    pub namespace: String,
    pub folder_binary: String,
    pub suffix_table: String,
    pub suffix_element: String,
    pub support_binary: bool,
    pub support_clone: bool,
    pub legacy_using: bool,
}
