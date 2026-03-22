use super::*;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use dejavu::Template;
use xcell_analyzer::XClassItem;
use xcell_types::codegen::TypeScriptWriter;

#[derive(Template)]
#[template(path = "BuildClass.ts.dejavu", escape = "none")]
pub struct CocosClass {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// Table name
    table_name: String,
    /// ID type
    id_type: &'static str,
    /// Cocos codegen configuration
    config: CocosCodegen,
    /// Key name
    key_name: String,
    /// Class fields
    class_fields: Vec<ClassField>,
}

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
    /// TypeScript writer code
    writer: TypeScriptWriter,
}

impl CocosCodegen {
    /// Writes Cocos class code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Class data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_class(&self, ws: &WorkspaceManager, table: &XClassData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        let out = self.make_class(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Cocos class template data
    ///
    /// # Arguments
    /// * `table` - Class data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// Cocos class template data
    fn make_class(&self, table: &XClassData, table_name: String) -> CocosClass {
        CocosClass {
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
        let default = self.typing.as_typescript_default();
        ClassField {
            document: self.document.lines(),
            name: self.field.clone(),
            typing: self.typing.as_typescript_type(),
            has_default: !default.is_empty(),
            default,
            getter: format!("get{}", self.field.to_case(Case::Pascal)),
            writer: self.typing.make_ts_writer(&self.field),
        }
    }
}
