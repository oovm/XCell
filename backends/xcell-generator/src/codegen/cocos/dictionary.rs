use super::*;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use dejavu::Template;
use xcell_analyzer::XCellHeader;
use xcell_provider::XCellAccess;
use xcell_types::codegen::TypeScriptWriter;

#[derive(Template)]
#[template(path = "BuildDictionary.ts.dejavu")]
pub struct CocosDictionaryTemplate {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// Table name
    table_name: String,
    /// ID type
    id_type: &'static str,
    /// Namespace
    namespace: String,
    /// Key name
    key_name: String,
    /// Class documentation
    class_document: Vec<String>,
    /// Dictionary fields
    class_fields: Vec<DictField>,
}

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
    /// TypeScript writer code
    writer: TypeScriptWriter,
}

impl CocosCodegen {
    /// Writes Cocos dictionary code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Dictionary data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        use nargo_types::NargoValue;
        
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        
        // 创建 NargoValue 上下文
        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        context_data.insert("table_name".to_string(), NargoValue::String(table_name.clone()));
        context_data.insert("id_type".to_string(), NargoValue::String("number".to_string()));
        context_data.insert("namespace".to_string(), NargoValue::String(self.namespace.clone()));
        context_data.insert("key_name".to_string(), NargoValue::String("key".to_string()));
        context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));
        
        // 处理 class_fields
        let class_fields: Vec<(Vec<String>, String, String, bool, String)> = table.headers.iter().map(|header| {
            let default = header.typing.as_typescript_default();
            (vec![], header.field_name.clone(), header.typing.as_typescript_type(), !default.is_empty(), default)
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
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        let out = loader.render_template(TemplateType::Class.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        
        Ok(())
    }

    /// Writes Cocos list code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - List data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        use nargo_types::NargoValue;
        
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        
        // 创建 NargoValue 上下文
        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        context_data.insert("table_name".to_string(), NargoValue::String(table_name.clone()));
        context_data.insert("id_type".to_string(), NargoValue::String("number".to_string()));
        context_data.insert("namespace".to_string(), NargoValue::String(self.namespace.clone()));
        context_data.insert("key_name".to_string(), NargoValue::String("id".to_string()));
        context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));
        
        // 处理 class_fields
        let class_fields: Vec<(Vec<String>, String, String, bool, String)> = table.headers.iter().map(|header| {
            let default = header.typing.as_typescript_default();
            (vec![], header.field_name.clone(), header.typing.as_typescript_type(), !default.is_empty(), default)
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
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        let out = loader.render_template(TemplateType::Class.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        
        Ok(())
    }
}


