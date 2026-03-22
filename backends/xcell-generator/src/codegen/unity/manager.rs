use super::*;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use xcell_analyzer::{UnityCodegen, WorkspaceManager};
use xcell_types::{XCellValue, XError, XResult};

#[derive(Template)]
#[template(path = "BuildManager.cs", escape = "none")]
pub struct UnityManagerTemplate {
    compiler_version: &'static str,
    data_version: String,
    edit_time: String,
    config: UnityCodegen,
    tables: Vec<TableField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableField {
    typing: String,
    public_name: String,
    private_name: String,
}

impl Display for TableField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub(super) fn write_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
        let unity = &ws.config.unity;
        let root = &ws.config.root;
        
        let output_dir = PathBuf::from(&unity.loader.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join("DataTableManager.cs");
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
    fn make_manager(&self, ws: &WorkspaceManager) -> UnityManager {
        UnityManager {
            compiler_version: env!("CARGO_PKG_VERSION"),
            data_version: "".to_string(),
            edit_time: "".to_string(),
            config: self.clone(),
            tables: Vec::new(),
        }
    }
}
