use crate::{WorkspaceManager, XCellHeader, XClassData, XDictData, XEnumerateData, XListData};
use dejavu_derive::Template;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use url::Url;
use xcell_types::{XError, XResult};

mod class;
mod dictionary;
mod enumerate;
mod manager;

#[derive(Clone, Debug, Serialize)]
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

impl Default for XluaCodegen {
    fn default() -> Self {
        XluaCodegen {
            enable: false,
            output: "xlua".to_string(),
            manager_name: "XCellManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "xcell".to_string(),
        }
    }
}

impl XluaCodegen {
    /// XLua output directory
    pub fn xlua_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = root.join(&self.output);
        let path = dir.join(file_name).with_extension("lua");
        Ok(path)
    }

    /// Manager path
    pub fn xlua_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.xlua_path(root, &self.manager_name)
    }

    /// Lua relative path
    pub fn xlua_lua_relative(&self, file_name: &str) -> String {
        format!("{}/{}.lua", self.output, file_name)
    }

    /// Ensure output directories exist
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.xlua_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }

    /// Write Lua code
    pub fn write_lua(&self, _ws: &WorkspaceManager) -> XResult<()> {
        // 暂时禁用 Lua 代码生成，因为缺少模板文件
        Ok(())
    }

    /// Log Lua file creation
    fn log_lua(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.xlua_path(&ws.config.root, name)?;
        tracing::info!("写入 Lua: {}\n{}", self.xlua_lua_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

impl super::Codegen for XluaCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        // TODO: Implement XLua code generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "xlua"
    }
}
