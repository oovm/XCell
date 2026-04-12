use super::*;
use xcell_analyzer::{XEnumerateData};
use crate::template::{TemplateLoader, TemplateType};
use nargo_types::NargoValue;

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

        let mut context_data = std::collections::HashMap::new();
        context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), NargoValue::String(table.name.clone()));

        let enumerate_ids: Vec<EnumeratePair> = table.lines.iter().map(|line| EnumeratePair {
            key: line.key.clone(),
            value: line.id.to_string(),
            document: vec![],
        }).collect();
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

        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;

        let out = loader.render_with_dejavu(TemplateType::Enumerate.file_name(), &context)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}


