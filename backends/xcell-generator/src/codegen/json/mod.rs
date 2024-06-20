use crate::{XResult, WorkspaceManager, XClassData, XDictData, XEnumerateData, XListData};
use serde::Serialize;
use serde_json::json;
use std::{fs::File, path::{Path, PathBuf}};
use url::Url;

#[derive(Clone, Debug, Serialize)]
pub struct JsonCodegen {
    /// Whether to generate JSON code
    pub enable: bool,
    /// Output directory
    pub output: String,
}

impl Default for JsonCodegen {
    fn default() -> Self {
        JsonCodegen { enable: false, output: "json".to_string() }
    }
}

impl JsonCodegen {
    /// JSON output directory
    pub fn json_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = root.join(&self.output);
        let path = dir.join(file_name).with_extension("json");
        Ok(path)
    }

    /// JSON relative path
    pub fn json_relative(&self, file_name: &str) -> String {
        format!("{}/{}.json", self.output, file_name)
    }

    /// Ensure output directories exist
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.json_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }

    /// Write JSON code
    pub fn write_json(&self, ws: &WorkspaceManager) -> XResult<()> {
        if !self.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

        for table in ws.classes() {
            if let Err(e) = self.write_class(ws, table) {
                tracing::error!("生成JSON类失败: {}", e);
            }
        }
        for table in ws.enumerates() {
            if let Err(e) = self.write_enumerate(ws, table) {
                tracing::error!("生成JSON枚举失败: {}", e);
            }
        }
        for table in ws.dicts() {
            if let Err(e) = self.write_dict(ws, table) {
                tracing::error!("生成JSON字典失败: {}", e);
            }
        }
        for table in ws.lists() {
            if let Err(e) = self.write_list(ws, table) {
                tracing::error!("生成JSON列表失败: {}", e);
            }
        }

        Ok(())
    }

    /// Write class JSON
    fn write_class(&self, _ws: &WorkspaceManager, _table: &XClassData) -> XResult<()> {
        // TODO: Implement write_class
        Ok(())
    }

    /// Write enumerate JSON
    fn write_enumerate(&self, _ws: &WorkspaceManager, _table: &XEnumerateData) -> XResult<()> {
        // TODO: Implement write_enumerate
        Ok(())
    }

    /// Write dict JSON
    fn write_dict(&self, _ws: &WorkspaceManager, _table: &XDictData) -> XResult<()> {
        // TODO: Implement write_dict
        Ok(())
    }

    /// Write list JSON
    fn write_list(&self, _ws: &WorkspaceManager, _table: &XListData) -> XResult<()> {
        // TODO: Implement write_list
        Ok(())
    }

    /// Make class JSON data
    fn make_class(&self, _table: &XClassData) -> serde_json::Value {
        let data = json!({});
        // TODO: Implement class JSON generation
        data
    }

    /// Make enumerate JSON data
    fn make_enumerate(&self, _table: &XEnumerateData) -> serde_json::Value {
        let data = json!({});
        // TODO: Implement enumerate JSON generation
        data
    }

    /// Make dict JSON data
    fn make_dict(&self, _table: &XDictData) -> serde_json::Value {
        let data = json!({});
        // TODO: Implement dict JSON generation
        data
    }

    /// Make list JSON data
    fn make_list(&self, _table: &XListData) -> serde_json::Value {
        let data = json!({});
        // TODO: Implement list JSON generation
        data
    }

    /// Log JSON file creation
    fn log_json(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.json_path(&ws.config.root, name)?;
        tracing::info!("写入 JSON: {}\n{}", self.json_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

impl super::Codegen for JsonCodegen {
    fn generate(&self, _context: &super::CodegenContext) -> XResult<()> {
        // TODO: Implement JSON code generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "json"
    }
}
