use super::*;
use xcell_analyzer::{XDictData, XListData};
use xcell_provider::{XCellAccess, XCellHeader};
use convert_case::{Case, Casing};
use crate::template::{TemplateLoader, TemplateType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictField {
    /// Field documentation
    document: Vec<String>,
    /// Field name
    name: String,
    /// Access modifier
    access: &'static str,
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
    /// Writes TypeScript dictionary code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Dictionary data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
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
        let class_fields_value: Vec<serde_json::Value> = table.headers.iter().map(|header| {
            let default = header.typing.as_typescript_default();
            let mut field_data = serde_json::Map::new();
            field_data.insert("document".to_string(), serde_json::Value::Array(
                header.document.lines().into_iter().map(|doc| serde_json::Value::String(doc)).collect()
            ));
            field_data.insert("name".to_string(), serde_json::Value::String(header.field_name.clone()));
            field_data.insert("typing".to_string(), serde_json::Value::String(header.typing.as_typescript_type()));
            field_data.insert("getter".to_string(), serde_json::Value::String(format!("get{}", header.field_name.to_case(Case::Pascal))));
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

    /// Writes TypeScript list code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - List data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        
        // 创建 serde_json::Value 上下文
        let mut context_data = serde_json::Map::new();
        context_data.insert("compiler_version".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), serde_json::Value::String(table.name.clone()));
        context_data.insert("table_name".to_string(), serde_json::Value::String(table_name.clone()));
        context_data.insert("key_name".to_string(), serde_json::Value::String("id".to_string()));
        context_data.insert("class_document".to_string(), serde_json::Value::Array(vec![]));
        
        // 处理 class_fields
        let class_fields_value: Vec<serde_json::Value> = table.headers.iter().map(|header| {
            let default = header.typing.as_typescript_default();
            let mut field_data = serde_json::Map::new();
            field_data.insert("document".to_string(), serde_json::Value::Array(
                header.document.lines().into_iter().map(|doc| serde_json::Value::String(doc)).collect()
            ));
            field_data.insert("name".to_string(), serde_json::Value::String(header.field_name.clone()));
            field_data.insert("typing".to_string(), serde_json::Value::String(header.typing.as_typescript_type()));
            field_data.insert("getter".to_string(), serde_json::Value::String(format!("get{}", header.field_name.to_case(Case::Pascal))));
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


