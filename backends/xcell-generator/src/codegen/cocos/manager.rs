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
        use nargo_types::NargoValue;
        
        let mut file = self.log_typescript(ws, &self.manager_name)?;
        
        // 创建 NargoValue 上下文
        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(self.manager_name.clone()));
        context_data.insert("instance_name".to_string(), NargoValue::String(self.instance_name.clone()));
        context_data.insert("namespace".to_string(), NargoValue::String(self.namespace.clone()));
        context_data.insert("manager_name".to_string(), NargoValue::String(self.manager_name.clone()));
        context_data.insert("data_version".to_string(), NargoValue::String("1.0.0".to_string()));
        context_data.insert("edit_time".to_string(), NargoValue::String(chrono::Utc::now().to_rfc3339()));
        
        // 添加 table_data_path
        let table_data_path = if self.table_data_path.is_empty() {
            "tables/"
        } else if self.table_data_path.ends_with('/') || self.table_data_path.ends_with('\\') {
            &self.table_data_path
        } else {
            &format!("{}/", self.table_data_path)
        };
        context_data.insert("table_data_path".to_string(), NargoValue::String(table_data_path.to_string()));
        
        // 处理 tables
        let mut tables = Vec::new();
        
        // Add class tables
        for table in ws.classes() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push((private_name, public_name, table_name));
        }
        
        // Add dict tables
        for table in ws.dicts() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push((private_name, public_name, table_name));
        }
        
        // Add list tables
        for table in ws.lists() {
            let table_name = format!("{}{}", table.name, self.suffix_table);
            let private_name = format!("{}Table", table.name.to_lowercase());
            let public_name = format!("get{}Table", table.name);
            tables.push((private_name, public_name, table_name));
        }
        
        let tables_value: Vec<NargoValue> = tables.iter().map(|(private_name, public_name, typing)| {
            let mut table_data = std::collections::HashMap::new();
            table_data.insert("private_name".to_string(), NargoValue::String(private_name.clone()));
            table_data.insert("public_name".to_string(), NargoValue::String(public_name.clone()));
            table_data.insert("typing".to_string(), NargoValue::String(typing.clone()));
            NargoValue::Object(table_data)
        }).collect();
        context_data.insert("tables".to_string(), NargoValue::Array(tables_value));
        
        let context = NargoValue::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        let out = loader.render_template(TemplateType::Manager.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        
        Ok(())
    }
}
