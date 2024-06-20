use super::*;

#[derive(Template)]
#[template(path = "BuildManager.lua", ext = "txt", escape = "none")]
pub struct XluaManager {
    /// Compiler version
    compiler_version: &'static str,
    /// XLua codegen configuration
    config: XluaCodegen,
    /// Enumerate tables
    enumerates: Vec<String>,
    /// Dictionary tables
    dicts: Vec<String>,
    /// List tables
    lists: Vec<String>,
    /// Class tables
    classes: Vec<String>,
}

impl XluaCodegen {
    /// Writes XLua manager code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_manager(&self, _ws: &WorkspaceManager) -> XResult<()> {
        // TODO: Implement write_manager
        Ok(())
    }

    /// Creates XLua manager template data
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// XLua manager template data
    fn make_manager(&self, ws: &WorkspaceManager) -> XluaManager {
        XluaManager {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            enumerates: ws.enumerates().map(|s| format!("{}{}", s.name, self.suffix_table)).collect(),
            dicts: ws.dicts().map(|s| format!("{}{}", s.name, self.suffix_table)).collect(),
            lists: ws.lists().map(|s| format!("{}{}", s.name, self.suffix_table)).collect(),
            classes: ws.classes().map(|s| format!("{}{}", s.name, self.suffix_table)).collect(),
        }
    }
}
