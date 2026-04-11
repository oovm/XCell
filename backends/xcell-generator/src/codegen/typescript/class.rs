use super::*;
use xcell_analyzer::{XClassData, XClassItem};
use convert_case::{Case, Casing};
use crate::template::{TemplateLoader, TemplateType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassField {
    /// Field documentation
    document: Vec<String>,
    /// Field name
    name: String,
    /// Field type
    typing: String,
    /// Getter method
    getter: String,
    /// Whether the field has a default value
    has_default: bool,
    /// Default value
    default: String,
}

impl TypeScriptCodegen {
    /// Writes TypeScript class code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Class data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_class(&self, ws: &WorkspaceManager, table: &XClassData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        
        // 创建 serde_json::Value 上下文
        let mut context_data = serde_json::Map::new();
        context_data.insert("compiler_version".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), serde_json::Value::String(table.name.clone()));
        context_data.insert("table_name".to_string(), serde_json::Value::String(table_name.clone()));
        context_data.insert("key_name".to_string(), serde_json::Value::String("key".to_string()));
        context_data.insert("class_document".to_string(), serde_json::Value::Array(vec![]));
        
        // 处理 class_fields
        let class_fields_value: Vec<serde_json::Value> = table.items.iter().map(|item| {
            let default = item.typing.as_typescript_default();
            let mut field_data = serde_json::Map::new();
            field_data.insert("document".to_string(), serde_json::Value::Array(
                item.document.lines().into_iter().map(|doc| serde_json::Value::String(doc)).collect()
            ));
            field_data.insert("name".to_string(), serde_json::Value::String(item.field.clone()));
            field_data.insert("typing".to_string(), serde_json::Value::String(item.typing.as_typescript_type()));
            field_data.insert("getter".to_string(), serde_json::Value::String(format!("get{}", item.field.to_case(Case::Pascal))));
            field_data.insert("has_default".to_string(), serde_json::Value::Bool(!default.is_empty()));
            field_data.insert("default".to_string(), serde_json::Value::String(default));
            serde_json::Value::Object(field_data)
        }).collect();
        context_data.insert("class_fields".to_string(), serde_json::Value::Array(class_fields_value));
        
        let context = serde_json::Value::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        let out = loader.render_template(TemplateType::Class.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}


