use super::*;

#[derive(Template)]
#[template(path = "BuildManager.ts", ext = "txt", escape = "none")]
pub struct CocosManager {
    /// Compiler version
    compiler_version: &'static str,
    /// Manager name
    class_name: String,
    /// Instance name
    instance_name: String,
    /// Cocos codegen configuration
    config: CocosCodegen,
    /// Class tables
    class_tables: Vec<String>,
    /// Enumerate tables
    enum_tables: Vec<String>,
    /// Dictionary tables
    dict_tables: Vec<String>,
    /// List tables
    list_tables: Vec<String>,
}

impl CocosCodegen {
    /// Writes Cocos manager code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
        let mut file = self.log_typescript(ws, &self.manager_name)?;
        let out = self.make_manager(ws).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Cocos manager template data
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Cocos manager template data
    fn make_manager(&self, ws: &WorkspaceManager) -> CocosManager {
        CocosManager {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            class_name: self.manager_name.clone(),
            instance_name: self.instance_name.clone(),
            class_tables: ws.classes().map(|t| format!("{}{}", t.name, self.suffix_table)).collect(),
            enum_tables: ws.enumerates().map(|t| t.name.clone()).collect(),
            dict_tables: ws.dicts().map(|t| format!("{}{}", t.name, self.suffix_table)).collect(),
            list_tables: ws.lists().map(|t| format!("{}{}", t.name, self.suffix_table)).collect(),
        }
    }
}
