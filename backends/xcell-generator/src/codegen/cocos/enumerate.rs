use super::*;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use xcell_analyzer::XEnumerateData;
use crate::template::{TemplateLoader, TemplateType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    /// Key
    key: String,
    /// Value
    value: String,
    /// Documentation
    document: Vec<String>,
}

impl CocosCodegen {
    /// Writes Cocos enumerate code
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
        context_data.insert("id_type".to_string(), NargoValue::String("number".to_string()));
        context_data.insert("namespace".to_string(), NargoValue::String(self.namespace.clone()));
        
        // 处理 enumerate_ids
        let enumerate_ids: Vec<(String, String, Vec<String>)> = table.lines.iter().map(|line| {
            (line.key.clone(), line.id.to_string(), vec![])
        }).collect();
        
        let enumerate_ids_value: Vec<NargoValue> = enumerate_ids.iter().map(|(key, value, document)| {
            let mut pair_data = std::collections::HashMap::new();
            pair_data.insert("key".to_string(), NargoValue::String(key.clone()));
            pair_data.insert("value".to_string(), NargoValue::String(value.clone()));
            pair_data.insert("document".to_string(), NargoValue::Array(
                document.iter().map(|doc| NargoValue::String(doc.clone())).collect()
            ));
            NargoValue::Object(pair_data)
        }).collect();
        context_data.insert("enumerate_ids".to_string(), NargoValue::Array(enumerate_ids_value));
        context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));
        
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


