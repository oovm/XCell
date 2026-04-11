use super::*;
use crate::template::{TemplateLoader, TemplateType};

#[derive(Template)]
#[template(path = "BuildEnumerate.ts", ext = "txt", escape = "none")]
pub struct TypeScriptEnumerate {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// TypeScript codegen configuration
    config: TypeScriptCodegen,
    /// Enumerate IDs
    enumerate_ids: Vec<EnumeratePair>,
    /// Enumerate fields
    enumerate_fields: Vec<EnumerateField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumerateField {
    /// Field documentation
    document: Vec<String>,
    /// Switch cases
    switch: Vec<EnumeratePair>,
    /// Field name
    name: String,
    /// Field type
    typing: String,
    /// Getter method
    getter: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    /// Key
    key: String,
    /// Value
    value: String,
    /// Documentation
    document: Vec<String>,
}

impl TypeScriptCodegen {
    /// Writes TypeScript enumerate code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_enumerate(&self, ws: &WorkspaceManager, table: &XEnumerateData) -> XResult<()> {
        use nargo_types::NargoValue;
        
        let mut file = self.log_typescript(ws, &table.name)?;
        
        // 创建 NargoValue 上下文
        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));
        
        // 处理 enumerate_ids
        let enumerate_ids = table.lines.iter().map(|data| data.as_enumerate()).collect::<Vec<EnumeratePair>>();
        let enumerate_ids_value: Vec<NargoValue> = enumerate_ids.iter().map(|pair| {
            let mut pair_data = std::collections::HashMap::new();
            pair_data.insert("key".to_string(), NargoValue::String(pair.key.clone()));
            pair_data.insert("value".to_string(), NargoValue::String(pair.value.clone()));
            pair_data.insert("document".to_string(), NargoValue::Array(
                pair.document.iter().map(|doc| NargoValue::String(doc.clone())).collect()
            ));
            NargoValue::Object(pair_data)
        }).collect();
        context_data.insert("enumerate_ids".to_string(), NargoValue::Array(enumerate_ids_value));
        
        let context = NargoValue::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        let out = loader.render_template(TemplateType::Enumerate.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

impl XCellHeader {
    /// Converts XCellHeader to EnumerateField
    ///
    /// # Arguments
    /// * `values` - Data lines
    /// * `index` - Field index
    ///
    /// # Returns
    /// EnumerateField representation
    fn as_enumerate(&self, values: &[XDataLine], index: usize) -> EnumerateField {
        EnumerateField {
            name: self.field_name.clone(),
            typing: self.typing.as_typescript_type(),
            getter: format!("get{}", self.field_name.to_case(Case::Pascal)),
            document: self.document.lines(),
            switch: values.iter().map(|data| data.as_pair(index)).collect(),
        }
    }
}

impl XDataLine {
    /// Converts XDataLine to EnumeratePair for IDs
    ///
    /// # Returns
    /// EnumeratePair representation
    fn as_enumerate(&self) -> EnumeratePair {
        EnumeratePair { key: self.key.clone(), value: self.id.to_string(), document: self.comment.lines() }
    }

    /// Converts XDataLine to EnumeratePair for field values
    ///
    /// # Arguments
    /// * `index` - Field index
    ///
    /// # Returns
    /// EnumeratePair representation
    fn as_pair(&self, index: usize) -> EnumeratePair {
        // 枚举和字段一样长, 必定存在
        let data = self.data.get(index).unwrap();
        EnumeratePair { key: self.key.clone(), value: data.as_typescript_value(), document: self.comment.lines() }
    }
}
