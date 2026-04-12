use super::*;
use xcell_analyzer::{XDictData, XListData};
use convert_case::{Case, Casing};
use crate::template::{TemplateLoader, TemplateType};
use crate::codegen::core::typescript::AsTypeScriptType;
use nargo_types::NargoValue;

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
        
        // 生成单个 Table 文件 (包含 Item 和 Table 类)
        let mut table_file = self.log_typescript(ws, &table_name)?;
        
        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        context_data.insert("table_name".to_string(), NargoValue::String(table_name.clone()));
        context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));

        let class_fields_value: Vec<NargoValue> = table.headers.iter().enumerate().map(|(index, header)| {
            let mut default = header.typing.as_typescript_default();
            let is_key = index == 0;
            
            // 对于主键字段，如果默认值为空，则设置类型默认值
            if is_key && default.is_empty() {
                let type_str = header.typing.as_typescript_type();
                default = match type_str.as_str() {
                    "number" => "0".to_string(),
                    "string" => "\"\"".to_string(),
                    "boolean" => "false".to_string(),
                    _ => "null".to_string(),
                };
            }
            
            let mut field_data = std::collections::HashMap::new();
            field_data.insert("document".to_string(), NargoValue::Array(
                header.document.lines().into_iter().map(|doc| NargoValue::String(doc)).collect()
            ));
            field_data.insert("name".to_string(), NargoValue::String(header.field_name.clone()));
            field_data.insert("typing".to_string(), NargoValue::String(header.typing.as_typescript_type()));
            field_data.insert("getter".to_string(), NargoValue::String(format!("get{}", header.field_name.to_case(Case::Pascal))));
            field_data.insert("has_default".to_string(), NargoValue::Bool(!default.is_empty()));
            field_data.insert("default".to_string(), NargoValue::String(default));
            field_data.insert("is_key".to_string(), NargoValue::Bool(is_key));
            NargoValue::Object(field_data)
        }).collect();
        context_data.insert("class_fields".to_string(), NargoValue::Array(class_fields_value));

        let context = NargoValue::Object(context_data);

        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;

        // 如果有自定义模板目录，使用 BuildClass.ts.dejavu，否则使用 BuildDictTable.ts.dejavu
        let template_name = if self.template_dir.is_some() {
            TemplateType::Class.file_name()
        } else {
            TemplateType::DictTable.file_name()
        };
        let out = loader.render_with_dejavu(template_name, &context)?;
        table_file.write_all(out.as_bytes())?;
        
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
        
        // 生成单个 Table 文件 (包含 Item 和 Table 类)
        let mut table_file = self.log_typescript(ws, &table_name)?;
        
        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        context_data.insert("table_name".to_string(), NargoValue::String(table_name.clone()));
        context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));

        let class_fields_value: Vec<NargoValue> = table.headers.iter().enumerate().map(|(index, header)| {
            let mut default = header.typing.as_typescript_default();
            let is_key = index == 0;
            
            // 对于主键字段，如果默认值为空，则设置类型默认值
            if is_key && default.is_empty() {
                let type_str = header.typing.as_typescript_type();
                default = match type_str.as_str() {
                    "number" => "0".to_string(),
                    "string" => "\"\"".to_string(),
                    "boolean" => "false".to_string(),
                    _ => "null".to_string(),
                };
            }
            
            let mut field_data = std::collections::HashMap::new();
            field_data.insert("document".to_string(), NargoValue::Array(
                header.document.lines().into_iter().map(|doc| NargoValue::String(doc)).collect()
            ));
            field_data.insert("name".to_string(), NargoValue::String(header.field_name.clone()));
            field_data.insert("typing".to_string(), NargoValue::String(header.typing.as_typescript_type()));
            field_data.insert("getter".to_string(), NargoValue::String(format!("get{}", header.field_name.to_case(Case::Pascal))));
            field_data.insert("has_default".to_string(), NargoValue::Bool(!default.is_empty()));
            field_data.insert("default".to_string(), NargoValue::String(default));
            field_data.insert("is_key".to_string(), NargoValue::Bool(is_key));
            NargoValue::Object(field_data)
        }).collect();
        context_data.insert("class_fields".to_string(), NargoValue::Array(class_fields_value));

        let context = NargoValue::Object(context_data);

        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;

        // 如果有自定义模板目录，使用 BuildClass.ts.dejavu，否则使用 BuildDictTable.ts.dejavu
        let template_name = if self.template_dir.is_some() {
            TemplateType::Class.file_name()
        } else {
            TemplateType::DictTable.file_name()
        };
        let out = loader.render_with_dejavu(template_name, &context)?;
        table_file.write_all(out.as_bytes())?;
        
        Ok(())
    }
}
