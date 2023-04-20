use crate::XDocument;
use super::*;

#[derive(Template)]
#[template(path = "BuildClass.cs.djv", ext = "txt", escape = "none")]
pub struct UnityClass {
    compiler_version: &'static str,
    class_name: String,
    table_name: String,
    id_type: &'static str,
    config: UnityCodegen,
    key_name: String,
    class_fields: Vec<ClassField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassField {
    document: Vec<String>,
    name: String,
    typing: String,
    getter: String,
    has_default: bool,
    default: String,
    reader: CSharpReader,
    writer: CSharpWriter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldReader {
    field: String,
    cast: String,
    is_vector: bool,
    properties: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldWriter {
    field: String,
    cast: String,
    is_vector: bool,
    properties: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    key: String,
    value: String,
    document: Vec<String>,
}

impl Display for ClassField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub(super) fn write_class(&self, ws: &WorkspaceManager, table: &XClassData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, ws.config.unity.suffix_table);
        let mut file = self.log_csharp(ws, &table_name)?;
        let out = self.make_class(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    fn make_class(&self, table: &XClassData, table_name: String) -> UnityClass {
        UnityClass {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "key".to_string(),
            id_type: "string",
            class_fields: table.items.iter().map(|s| s.as_dict()).collect(),
        }
    }
}

impl XClassItem {
    fn as_dict(&self) -> ClassField {
        let default = self.typing.as_csharp_default();
        ClassField {
            document: self.document.lines(),
            name: self.field.clone(),
            typing: self.typing.as_csharp_type(),
            has_default: !default.is_empty(),
            default,
            getter: "<getter>".to_string(),
            reader: self.typing.make_cs_binary_reader(&self.field),
            writer: self.typing.make_cs_binary_writer(&self.field),
        }
    }
}
