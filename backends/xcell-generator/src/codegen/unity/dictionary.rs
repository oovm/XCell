use super::*;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use xcell_analyzer::{UnityCodegen, WorkspaceManager, XCellHeader, XDictData, XListData};
use xcell_provider::XCellAccess;
use xcell_types::{
    XResult,
    codegen::{CSharpReader, CSharpWriter},
};

/// Unity dictionary code generation template
#[derive(Template)]
#[template(path = "BuildDictionary.cs", ext = "txt", escape = "none")]
pub struct UnityDictionary {
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
    /// Dictionary fields
    class_fields: Vec<DictField>,
}

/// Dictionary field information for code generation
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

impl Display for DictField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    /// Writes Unity dictionary code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Dictionary data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        let unity = &ws.config.unity;
        let root = &ws.config.root;
        
        let table_name = format!("{}{}", table.name, unity.loader.suffix_table);
        
        let output_dir = PathBuf::from(&unity.loader.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join(format!("{}.cs", table_name));
        let mut file = std::fs::File::create(path)?;
        let out = self.make_dict(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Unity dictionary template data
    ///
    /// # Arguments
    /// * `table` - Dictionary data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// Unity dictionary template data
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
    /// Writes Unity list code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - List data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        let unity = &ws.config.unity;
        let root = &ws.config.root;
        
        let table_name = format!("{}{}", table.name, unity.loader.suffix_table);
        
        let output_dir = PathBuf::from(&unity.loader.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join(format!("{}.cs", table_name));
        let mut file = std::fs::File::create(path)?;
        let out = self.make_list(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Unity list template data
    ///
    /// # Arguments
    /// * `table` - List data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// Unity dictionary template data (used for lists)
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
    /// Converts XCellHeader to DictField
    ///
    /// # Returns
    /// DictField representation
    fn as_dict(&self) -> DictField {
        let default = self.typing.as_csharp_default();
        let access = match self.access {
            XCellAccess::Default => "",
            XCellAccess::Public => "public ",
            XCellAccess::Private => "private ",
        };
        DictField {
            document: self.document.lines(),
            name: self.field_name.clone(),
            access,
            typing: self.typing.as_csharp_type(),
            has_default: !default.is_empty(),
            default,
            getter: "<getter>".to_string(),
            reader: self.typing.make_cs_binary_reader(&self.field_name),
            writer: self.typing.make_cs_binary_writer(&self.field_name),
        }
    }
}
