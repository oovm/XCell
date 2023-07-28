use super::*;

#[derive(Template)]
#[template(path = "BuildDictionary.cs", ext = "txt", escape = "none")]
pub struct UnityDictionary {
    compiler_version: &'static str,
    class_name: String,
    table_name: String,
    id_type: &'static str,
    config: UnityCodegen,
    key_name: String,
    class_fields: Vec<DictField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictField {
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

impl Display for DictField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, ws.config.unity.suffix_table);
        let mut file = self.log_csharp(ws, &table_name)?;
        let out = self.make_dict(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    fn make_dict(&self, table: &XDictData, table_name: String) -> UnityDictionary {
        UnityDictionary {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "key".to_string(),
            id_type: "string",
            class_fields: table.headers.iter().map(|s| s.as_dict()).collect(),
        }
    }
}

impl UnityCodegen {
    pub(super) fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, ws.config.unity.suffix_table);
        let mut file = self.log_csharp(ws, &table_name)?;
        let out = self.make_list(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    fn make_list(&self, table: &XListData, table_name: String) -> UnityDictionary {
        UnityDictionary {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "id".to_string(),
            id_type: table.id_type.as_csharp_type(),
            class_fields: table.headers.iter().map(|s| s.as_dict()).collect(),
        }
    }
}

impl XCellHeader {
    fn as_dict(&self) -> DictField {
        let default = self.typing.as_csharp_default();
        DictField {
            document: self.document.lines(),
            name: self.field_name.clone(),
            typing: self.typing.as_csharp_type(),
            has_default: !default.is_empty(),
            default,
            getter: "<getter>".to_string(),
            reader: self.typing.make_cs_binary_reader(&self.field_name),
            writer: self.typing.make_cs_binary_writer(&self.field_name),
        }
    }
}
