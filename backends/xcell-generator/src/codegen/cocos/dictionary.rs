use super::*;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use dejavu::Template;
use xcell_analyzer::XCellHeader;
use xcell_provider::XCellAccess;
use xcell_types::codegen::TypeScriptWriter;

#[derive(Template)]
#[template(path = "BuildDictionary.ts", ext = "txt", escape = "none")]
pub struct CocosDictionary {
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

impl CocosCodegen {
    /// Writes Cocos dictionary code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Dictionary data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        let out = self.make_dict(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Cocos dictionary template data
    ///
    /// # Arguments
    /// * `table` - Dictionary data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// Cocos dictionary template data
    fn make_dict(&self, table: &XDictData, table_name: String) -> CocosDictionary {
        CocosDictionary {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "key".to_string(),
            id_type: "string",
            class_fields: table.headers.iter().map(|s| s.as_dict()).collect(),
        }
    }

    /// Writes Cocos list code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - List data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        let table_name = format!("{}{}", table.name, self.suffix_table);
        let mut file = self.log_typescript(ws, &table_name)?;
        let out = self.make_list(table, table_name).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Cocos list template data
    ///
    /// # Arguments
    /// * `table` - List data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// Cocos dictionary template data (used for lists)
    fn make_list(&self, table: &XListData, table_name: String) -> CocosDictionary {
        CocosDictionary {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "id".to_string(),
            id_type: table.id_type.as_typescript_type(),
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
