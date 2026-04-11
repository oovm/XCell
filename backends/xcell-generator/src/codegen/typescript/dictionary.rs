use super::*;
use xcell_analyzer::XCellHeader;
use xcell_provider::XCellAccess;
use convert_case::{Case, Casing};
use xcell_types::codegen::TypeScriptWriter;
use crate::template::{TemplateLoader, TemplateType};

#[derive(Template)]
#[template(path = "BuildDictionary.ts", ext = "txt", escape = "none")]
pub struct TypeScriptDictionary {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// Table name
    table_name: String,
    /// TypeScript codegen configuration
    config: TypeScriptCodegen,
    /// Key name
    key_name: String,
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
        use nargo_types::NargoValue;
        
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        
        // 创建 NargoValue 上下文
        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        context_data.insert("table_name".to_string(), NargoValue::String(table_name.clone()));
        context_data.insert("key_name".to_string(), NargoValue::String("key".to_string()));
        
        // 处理 class_fields
        let class_fields: Vec<DictField> = table.headers.iter().map(|s| s.as_dict()).collect();
        let class_fields_value: Vec<NargoValue> = class_fields.iter().map(|field| {
            let mut field_data = std::collections::HashMap::new();
            field_data.insert("document".to_string(), NargoValue::Array(
                field.document.iter().map(|doc| NargoValue::String(doc.clone())).collect()
            ));
            field_data.insert("name".to_string(), NargoValue::String(field.name.clone()));
            field_data.insert("typing".to_string(), NargoValue::String(field.typing.clone()));
            field_data.insert("getter".to_string(), NargoValue::String(field.getter.clone()));
            field_data.insert("has_default".to_string(), NargoValue::Bool(field.has_default));
            field_data.insert("default".to_string(), NargoValue::String(field.default.clone()));
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

    /// Writes TypeScript list code
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
        context_data.insert("key_name".to_string(), NargoValue::String("id".to_string()));
        
        // 处理 class_fields
        let class_fields: Vec<DictField> = table.headers.iter().map(|s| s.as_dict()).collect();
        let class_fields_value: Vec<NargoValue> = class_fields.iter().map(|field| {
            let mut field_data = std::collections::HashMap::new();
            field_data.insert("document".to_string(), NargoValue::Array(
                field.document.iter().map(|doc| NargoValue::String(doc.clone())).collect()
            ));
            field_data.insert("name".to_string(), NargoValue::String(field.name.clone()));
            field_data.insert("typing".to_string(), NargoValue::String(field.typing.clone()));
            field_data.insert("getter".to_string(), NargoValue::String(field.getter.clone()));
            field_data.insert("has_default".to_string(), NargoValue::Bool(field.has_default));
            field_data.insert("default".to_string(), NargoValue::String(field.default.clone()));
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

impl XCellHeader {
    /// Converts XCellHeader to DictField
    ///
    /// # Returns
    /// DictField representation
    fn as_dict(&self) -> DictField {
        let default = self.typing.as_typescript_default();
        let access = match self.access {
            XCellAccess::Default => "",
            XCellAccess::Public => "public ",
            XCellAccess::Private => "private ",
        };
        DictField {
            document: self.document.lines(),
            name: self.field_name.clone(),
            access,
            typing: self.typing.as_typescript_type(),
            has_default: !default.is_empty(),
            default,
            getter: format!("get{}", self.field_name.to_case(Case::Pascal)),
            writer: self.typing.make_ts_writer(&self.field_name),
        }
    }
}
