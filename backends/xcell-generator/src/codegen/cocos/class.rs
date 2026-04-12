use super::*;
use xcell_analyzer::XClassData;
use crate::template::{TemplateLoader, TemplateType};
use nargo_types::NargoValue;

impl CocosCodegen {
    /// Writes Cocos class code
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

        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        context_data.insert("table_name".to_string(), NargoValue::String(table_name.clone()));
        context_data.insert("id_type".to_string(), NargoValue::String("number".to_string()));
        context_data.insert("namespace".to_string(), NargoValue::String(self.namespace.clone()));
        context_data.insert("key_name".to_string(), NargoValue::String("id".to_string()));
        context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));

        let class_fields: Vec<(Vec<String>, String, String, bool, String)> = table.items.iter().map(|item| {
            let default = item.typing.as_typescript_default();
            (vec![], item.field.clone(), item.typing.as_typescript_type(), !default.is_empty(), default)
        }).collect();

        let class_fields_value: Vec<NargoValue> = class_fields.iter().map(|(document, name, typing, has_default, default)| {
            let mut field_data = std::collections::HashMap::new();
            field_data.insert("document".to_string(), NargoValue::Array(
                document.iter().map(|doc| NargoValue::String(doc.clone())).collect()
            ));
            field_data.insert("name".to_string(), NargoValue::String(name.clone()));
            field_data.insert("typing".to_string(), NargoValue::String(typing.clone()));
            field_data.insert("has_default".to_string(), NargoValue::Bool(*has_default));
            field_data.insert("default".to_string(), NargoValue::String(default.clone()));
            NargoValue::Object(field_data)
        }).collect();
        context_data.insert("class_fields".to_string(), NargoValue::Array(class_fields_value));

        let context = NargoValue::Object(context_data);

        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;

        let out = loader.render_with_dejavu(TemplateType::Class.file_name(), &context)?;
        file.write_all(out.as_bytes())?;

        Ok(())
    }
}


