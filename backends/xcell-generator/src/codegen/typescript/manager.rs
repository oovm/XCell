use super::*;
use crate::template::{TemplateLoader, TemplateType};

#[derive(Template)]
#[template(path = "BuildManager.ts", ext = "txt", escape = "none")]
pub struct TypeScriptManager {
    /// Compiler version
    compiler_version: &'static str,
    /// Manager name
    class_name: String,
    /// Instance name
    instance_name: String,
    /// TypeScript codegen configuration
    config: TypeScriptCodegen,
    /// Class tables
    class_tables: Vec<String>,
    /// Enumerate tables
    enum_tables: Vec<String>,
    /// Dictionary tables
    dict_tables: Vec<String>,
    /// List tables
    list_tables: Vec<String>,
}

impl TypeScriptCodegen {
    /// Writes TypeScript manager code
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
        
        // 处理 tables
        let table_items = ws.classes()
            .map(|t| format!("{}{}", t.name, self.suffix_table))
            .chain(ws.dicts().map(|t| format!("{}{}", t.name, self.suffix_table)))
            .chain(ws.lists().map(|t| format!("{}{}", t.name, self.suffix_table)))
            .collect::<Vec<String>>();
        
        let tables_value: Vec<NargoValue> = table_items.iter().map(|table| {
            let mut table_data = std::collections::HashMap::new();
            table_data.insert("typing".to_string(), NargoValue::String(table.clone()));
            table_data.insert("private_name".to_string(), NargoValue::String(table.to_lowercase()));
            table_data.insert("public_name".to_string(), NargoValue::String(format!("get{}", table)));
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
