use super::*;
use xcell_analyzer::XClassData;
use convert_case::{Case, Casing};
use crate::template::{TemplateLoader, TemplateType};
use nargo_types::NargoValue;

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
        let data_name = format!("{}Data", table.name);
        let mut file = self.log_typescript(ws, &data_name)?;

        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        context_data.insert("table_name".to_string(), NargoValue::String(data_name.clone()));
        context_data.insert("key_name".to_string(), NargoValue::String("key".to_string()));
        context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));

        let class_fields_value: Vec<NargoValue> = table.items.iter().map(|item| {
            let default = item.typing.as_typescript_default();
            let mut field_data = std::collections::HashMap::new();
            field_data.insert("document".to_string(), NargoValue::Array(
                item.document.lines().into_iter().map(|doc| NargoValue::String(doc)).collect()
            ));
            field_data.insert("name".to_string(), NargoValue::String(item.field.clone()));
            field_data.insert("typing".to_string(), NargoValue::String(item.typing.as_typescript_type()));
            field_data.insert("getter".to_string(), NargoValue::String(format!("get{}", item.field.to_case(Case::Pascal))));
            field_data.insert("has_default".to_string(), NargoValue::Bool(!default.is_empty()));
            field_data.insert("default".to_string(), NargoValue::String(default));
            NargoValue::Object(field_data)
        }).collect();
        context_data.insert("class_fields".to_string(), NargoValue::Array(class_fields_value));

        let context = NargoValue::Object(context_data);

        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;

        let out = loader.render_with_dejavu(TemplateType::Data.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}


