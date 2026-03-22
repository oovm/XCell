use super::*;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use xcell_analyzer::WorkspaceManager;
use xcell_config::UnityCodegen;
use xcell_types::XResult;

/// Unity manager code generation template
#[derive(Template)]
#[template(path = "BuildManager.cs.dejavu", escape = "none")]
pub struct UnityManagerTemplate {
    /// Compiler version
    compiler_version: &'static str,
    /// Data version
    data_version: String,
    /// Edit time
    edit_time: String,
    /// Unity codegen configuration
    config: UnityCodegen,
    /// Tables
    tables: Vec<TableField>,
}

/// Table field information for manager
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableField {
    /// Table type
    typing: String,
    /// Public name
    public_name: String,
    /// Private name
    private_name: String,
}

impl Display for TableField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    /// Writes Unity manager code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Result of the operation
    pub fn write_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
        let root = &ws.config.root;
        
        let output_dir = self.loader_path(root);
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join(format!("{}.cs", self.manager));
        let mut file = std::fs::File::create(path)?;
        let out = self.make_manager(ws).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Unity manager template data
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Unity manager template data
    fn make_manager(&self, ws: &WorkspaceManager) -> UnityManagerTemplate {
        UnityManagerTemplate {
            compiler_version: env!("CARGO_PKG_VERSION"),
            data_version: String::new(),
            edit_time: String::new(),
            config: self.clone(),
            tables: Vec::new(),
        }
    }
}
