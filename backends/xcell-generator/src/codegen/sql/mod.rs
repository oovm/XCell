use crate::{WorkspaceManager, XCellHeader, XClassData, XDictData, XEnumerateData, XListData};
use dejavu_derive::Template;
use std::{fs::File, io::Write, path::Path};
use xcell_errors::{XError, XResult, for_3rd::Url};

// mod class;
// mod dictionary;
// mod enumerate;
// mod manager;

#[derive(Clone, Debug, Serialize)]
pub struct SqlCodegen {
    /// Whether to generate SQL code
    pub enable: bool,
    /// Output directory
    pub output: String,
    /// SQL dialect (mysql, postgresql, sqlite)
    pub dialect: String,
    /// Generated manager name
    pub manager_name: String,
    /// Generated table name suffix
    pub suffix_table: String,
}

impl Default for SqlCodegen {
    fn default() -> Self {
        SqlCodegen {
            enable: false,
            output: "sql".to_string(),
            dialect: "mysql".to_string(),
            manager_name: "XCellManager".to_string(),
            suffix_table: "Table".to_string(),
        }
    }
}

impl SqlCodegen {
    /// SQL output directory
    pub fn sql_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = root.join(&self.output);
        let path = dir.join(file_name).with_extension("sql");
        Ok(path)
    }

    /// Manager path
    pub fn sql_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.sql_path(root, &self.manager_name)
    }

    /// SQL relative path
    pub fn sql_relative(&self, file_name: &str) -> String {
        format!("{}/{}.sql", self.output, file_name)
    }

    /// Ensure output directories exist
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.sql_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }

    /// Write SQL code
    pub fn write_sql(&self, _ws: &WorkspaceManager) -> XResult<()> {
        if !self.enable {
            return Ok(());
        }

        // TODO: Implement SQL code generation
        Ok(())
    }

    /// Write class SQL
    fn write_class(&self, _ws: &WorkspaceManager, _table: &XClassData) -> XResult<()> {
        // TODO: Implement write_class
        Ok(())
    }

    /// Write enumerate SQL
    fn write_enumerate(&self, _ws: &WorkspaceManager, _table: &XEnumerateData) -> XResult<()> {
        // TODO: Implement write_enumerate
        Ok(())
    }

    /// Write dict SQL
    fn write_dict(&self, _ws: &WorkspaceManager, _table: &XDictData) -> XResult<()> {
        // TODO: Implement write_dict
        Ok(())
    }

    /// Write list SQL
    fn write_list(&self, _ws: &WorkspaceManager, _table: &XListData) -> XResult<()> {
        // TODO: Implement write_list
        Ok(())
    }

    /// Write manager SQL
    fn write_manager(&self, _ws: &WorkspaceManager) -> XResult<()> {
        // TODO: Implement write_manager
        Ok(())
    }

    /// Log SQL file creation
    fn log_sql(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.sql_path(&ws.config.root, name)?;
        tracing::info!("写入 SQL: {}\n{}", self.sql_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

impl super::Codegen for SqlCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        // TODO: Implement SQL code generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "sql"
    }
}
