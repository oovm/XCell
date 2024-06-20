use super::*;
use convert_case::Case;
use xcell_analyzer::{WorkspaceManager, XCellHeader, XDictData, XListData};
use xcell_provider::XCellAccess;
use xcell_types::XResult;

#[derive(Template)]
#[template(path = "BuildDictionary.lua", ext = "txt", escape = "none")]
pub struct XluaDictionary {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// Table name
    table_name: String,
    /// XLua codegen configuration
    config: XluaCodegen,
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
}

impl XluaCodegen {
    /// Writes XLua dictionary code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Dictionary data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_dict(&self, _ws: &WorkspaceManager, _table: &XDictData) -> XResult<()> {
        // TODO: Implement write_dict
        Ok(())
    }

    /// Creates XLua dictionary template data
    ///
    /// # Arguments
    /// * `table` - Dictionary data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// XLua dictionary template data
    fn make_dict(&self, table: &XDictData, table_name: String) -> XluaDictionary {
        XluaDictionary {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "key".to_string(),
            class_fields: table.headers.iter().map(|s| s.as_dict()).collect(),
        }
    }

    /// Writes XLua list code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - List data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_list(&self, _ws: &WorkspaceManager, _table: &XListData) -> XResult<()> {
        // TODO: Implement write_list
        Ok(())
    }

    /// Creates XLua list template data
    ///
    /// # Arguments
    /// * `table` - List data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// XLua dictionary template data (used for lists)
    fn make_list(&self, table: &XListData, table_name: String) -> XluaDictionary {
        XluaDictionary {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "id".to_string(),
            class_fields: table.headers.iter().map(|s| s.as_dict()).collect(),
        }
    }
}

impl XCellHeader {
    /// Converts XCellHeader to DictField
    ///
    /// # Returns
    /// DictField representation
    pub fn as_dict(&self) -> DictField {
        let default = "";
        let access = "";
        DictField {
            document: vec![],
            name: self.field_name.clone(),
            access,
            typing: "string".to_string(),
            has_default: false,
            default: default.to_string(),
            getter: format!("get{}", self.field_name),
        }
    }
}
