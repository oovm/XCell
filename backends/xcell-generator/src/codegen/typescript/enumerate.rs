use super::*;
use xcell_analyzer::{XEnumerateData};
use convert_case::{Case, Casing};
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
        let mut file = self.log_typescript(ws, &table.name)?;
        
        // 创建 serde_json::Value 上下文
        let mut context_data = serde_json::Map::new();
        context_data.insert("compiler_version".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), serde_json::Value::String(table.name.clone()));
        
        // 处理 enumerate_ids
        let enumerate_ids: Vec<EnumeratePair> = table.lines.iter().map(|line| EnumeratePair {
            key: line.key.clone(),
            value: line.id.to_string(),
            document: vec![],
        }).collect();
        let enumerate_ids_value: Vec<serde_json::Value> = enumerate_ids.iter().map(|pair| {
            let mut pair_data = serde_json::Map::new();
            pair_data.insert("key".to_string(), serde_json::Value::String(pair.key.clone()));
            pair_data.insert("value".to_string(), serde_json::Value::String(pair.value.clone()));
            pair_data.insert("document".to_string(), serde_json::Value::Array(
                pair.document.iter().map(|doc| serde_json::Value::String(doc.clone())).collect()
            ));
            serde_json::Value::Object(pair_data)
        }).collect();
        context_data.insert("enumerate_ids".to_string(), serde_json::Value::Array(enumerate_ids_value));
        
        let context = serde_json::Value::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        let out = loader.render_template(TemplateType::Enumerate.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}


