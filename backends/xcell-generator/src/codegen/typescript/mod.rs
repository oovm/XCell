use crate::{WorkspaceManager, XClassData, XDictData, XEnumerateData, XListData};
use dejavu_derive::Template;
use std::{fs::File, io::Write, path::Path};
use xcell_errors::{XError, XResult, for_3rd::Url};

mod class;
mod dictionary;
mod enumerate;
mod manager;

#[derive(Clone, Debug, Serialize)]
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
    /// Data format (json, csv)
    pub format: String,
}

impl Default for TypeScriptCodegen {
    fn default() -> Self {
        TypeScriptCodegen {
            enable: false,
            output: "typescript".to_string(),
            manager_name: "XCellManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "xcell".to_string(),
            format: "json".to_string(),
        }
    }
}

impl TypeScriptCodegen {
    /// TypeScript output directory
    pub fn ts_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = root.join(&self.output);
        let path = dir.join(file_name).with_extension("ts");
        Ok(path)
    }

    /// Manager path
    pub fn ts_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.ts_path(root, &self.manager_name)
    }

    /// TypeScript relative path
    pub fn ts_relative(&self, file_name: &str) -> String {
        format!("{}/{}.ts", self.output, file_name)
    }

    /// Ensure output directories exist
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.ts_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }

    /// Write TypeScript code
    pub fn write_typescript(&self, ws: &WorkspaceManager) -> XResult<()> {
        if !self.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

        for table in ws.classes() {
            if let Err(e) = self.write_class(ws, table) {
                tracing::error!("生成TypeScript类失败: {}", e);
            }
        }
        for table in ws.enumerates() {
            if let Err(e) = self.write_enumerate(ws, table) {
                tracing::error!("生成TypeScript枚举失败: {}", e);
            }
        }
        for table in ws.dicts() {
            if let Err(e) = self.write_dict(ws, table) {
                tracing::error!("生成TypeScript字典失败: {}", e);
            }
        }
        for table in ws.lists() {
            if let Err(e) = self.write_list(ws, table) {
                tracing::error!("生成TypeScript列表失败: {}", e);
            }
        }

        self.write_manager(ws)?;
        Ok(())
    }

    /// Log TypeScript file creation
    fn log_typescript(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.ts_path(&ws.config.root, name)?;
        tracing::info!("写入 TypeScript: {}\n{}", self.ts_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

impl super::Codegen for TypeScriptCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        // TODO: Implement TypeScript code generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "typescript"
    }
}
