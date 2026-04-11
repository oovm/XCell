use super::*;
use crate::template::{TemplateLoader, TemplateType};

impl TypeScriptCodegen {
    /// Writes TypeScript manager code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
        let mut file = self.log_typescript(ws, &self.manager_name)?;
        
        // 创建 serde_json::Value 上下文
        let mut context_data = serde_json::Map::new();
        context_data.insert("compiler_version".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), serde_json::Value::String(self.manager_name.clone()));
        context_data.insert("instance_name".to_string(), serde_json::Value::String(self.instance_name.clone()));
        
        // 处理 tables
        let table_items = ws.classes()
            .map(|t| format!("{}{}", t.name, self.suffix_table))
            .chain(ws.dicts().map(|t| format!("{}{}", t.name, self.suffix_table)))
            .chain(ws.lists().map(|t| format!("{}{}", t.name, self.suffix_table)))
            .collect::<Vec<String>>();
        
        let tables_value: Vec<serde_json::Value> = table_items.iter().map(|table| {
            let mut table_data = serde_json::Map::new();
            table_data.insert("typing".to_string(), serde_json::Value::String(table.clone()));
            table_data.insert("private_name".to_string(), serde_json::Value::String(table.to_lowercase()));
            table_data.insert("public_name".to_string(), serde_json::Value::String(format!("get{}", table)));
            serde_json::Value::Object(table_data)
        }).collect();
        context_data.insert("tables".to_string(), serde_json::Value::Array(tables_value));
        
        let context = serde_json::Value::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        let out = loader.render_template(TemplateType::Manager.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}
