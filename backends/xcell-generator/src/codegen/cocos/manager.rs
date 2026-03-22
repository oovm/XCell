use super::*;
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use dejavu::Template;
use chrono;

#[derive(Template)]
#[template(path = "BuildManager.ts.dejavu")]
pub struct CocosManagerTemplate {
    /// Compiler version
    compiler_version: &'static str,
    /// Manager name
    class_name: String,
    /// Instance name
    instance_name: String,
    /// Namespace
    namespace: String,
    /// Manager name
    manager_name: String,
    /// Data version
    data_version: String,
    /// Edit time
    edit_time: String,
    /// Tables
    tables: Vec<TableItem>,
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
        let out = self.make_manager(ws).render(&dejavu_types::values::Context::new())?;
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
    fn make_manager(&self, ws: &WorkspaceManager) -> CocosManagerTemplate {
        let mut tables = Vec::new();
        
        // Add class tables
        for table in ws.classes() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push(TableItem {
                private_name,
                public_name,
                typing: table_name,
            });
        }
        
        // Add dict tables
        for table in ws.dicts() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push(TableItem {
                private_name,
                public_name,
                typing: table_name,
            });
        }
        
        // Add list tables
        for table in ws.lists() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push(TableItem {
                private_name,
                public_name,
                typing: table_name,
            });
        }
        
        CocosManagerTemplate {
            compiler_version: env!("CARGO_PKG_VERSION"),
            namespace: self.namespace.clone(),
            class_name: self.manager_name.clone(),
            instance_name: self.instance_name.clone(),
            manager_name: self.manager_name.clone(),
            data_version: "1.0.0".to_string(),
            edit_time: chrono::Utc::now().to_rfc3339(),
            tables,
        }
    }
}
