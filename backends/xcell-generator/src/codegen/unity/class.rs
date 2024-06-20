use super::*;
use std::fmt::{Debug, Display, Formatter};
use xcell_analyzer::{UnityCodegen, WorkspaceManager, XClassData, XClassItem};
use xcell_types::{
    XResult,
    codegen::{CSharpReader, CSharpWriter},
};

/// Unity class code generation template
#[derive(Template)]
#[template(path = "BuildClass.cs", ext = "txt", escape = "none")]
pub struct UnityClass {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// Table name
    table_name: String,
    /// ID type
    id_type: &'static str,
    /// Unity codegen configuration
    config: UnityCodegen,
    /// Key name
    key_name: String,
    /// Class fields
    class_fields: Vec<ClassField>,
}

/// Class field information for code generation
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
    /// C# reader code
    reader: CSharpReader,
    /// C# writer code
    writer: CSharpWriter,
}

/// Field reader information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldReader {
    /// Field name
    field: String,
    /// Cast type
    cast: String,
    /// Whether the field is a vector
    is_vector: bool,
    /// Properties
    properties: Vec<String>,
}

/// Field writer information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldWriter {
    /// Field name
    field: String,
    /// Cast type
    cast: String,
    /// Whether the field is a vector
    is_vector: bool,
    /// Properties
    properties: Vec<String>,
}

/// Enumerate pair information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    /// Key
    key: String,
    /// Value
    value: String,
    /// Documentation
    document: Vec<String>,
}

impl Display for ClassField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    /// Writes Unity class code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Class data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_class(&self, ws: &WorkspaceManager, table: &XClassData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, ws.config.unity.suffix_table);
        let mut file = self.log_csharp(ws, &table_name)?;
        let out = self.make_class(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Unity class template data
    ///
    /// # Arguments
    /// * `table` - Class data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// Unity class template data
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
    /// Converts XClassItem to ClassField
    ///
    /// # Returns
    /// ClassField representation
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
