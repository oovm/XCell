use super::*;
use crate::template::{TemplateLoader, TemplateType};
use nargo_types::NargoValue;

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

        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(self.manager_name.clone()));
        context_data.insert("instance_name".to_string(), NargoValue::String(self.instance_name.clone()));
        context_data.insert("data_version".to_string(), NargoValue::String("1.0.0".to_string()));
        context_data.insert("edit_time".to_string(), NargoValue::String(chrono::Utc::now().to_rfc3339()));

        let tables = self.collect_tables(ws);
        let tables_value: Vec<NargoValue> = tables.iter().map(|table| {
            let mut table_data = std::collections::HashMap::new();
            table_data.insert("private_name".to_string(), NargoValue::String(table.private_name.clone()));
            table_data.insert("public_name".to_string(), NargoValue::String(table.public_name.clone()));
            table_data.insert("typing".to_string(), NargoValue::String(table.typing.clone()));
            NargoValue::Object(table_data)
        }).collect();
        context_data.insert("tables".to_string(), NargoValue::Array(tables_value));

        let context = NargoValue::Object(context_data);

        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;

        let out = loader.render_with_dejavu(TemplateType::Manager.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Collects all tables from workspace
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Vector of table items
    fn collect_tables(&self, ws: &WorkspaceManager) -> Vec<TableItem> {
        let mut tables = Vec::new();

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

        tables
    }
}

/// Table item struct
#[derive(Debug, Clone)]
pub struct TableItem {
    /// Private table name
    pub private_name: String,
    /// Public table name
    pub public_name: String,
    /// Table typing
    pub typing: String,
}

