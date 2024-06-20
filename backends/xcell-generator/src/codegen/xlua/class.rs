use super::*;
use convert_case::Case;
use xcell_analyzer::{WorkspaceManager, XClassData, XClassItem};
use xcell_types::XResult;

#[derive(Template)]
#[template(path = "BuildClass.lua", ext = "txt", escape = "none")]
pub struct XluaClass {
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
}

impl XluaCodegen {
    /// Writes XLua class code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Class data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_class(&self, _ws: &WorkspaceManager, _table: &XClassData) -> XResult<()> {
        // TODO: Implement write_class
        Ok(())
    }

    /// Creates XLua class template data
    ///
    /// # Arguments
    /// * `table` - Class data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// XLua class template data
    fn make_class(&self, table: &XClassData, table_name: String) -> XluaClass {
        XluaClass {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            key_name: "key".to_string(),
            class_fields: table.items.iter().map(|s| s.as_dict()).collect(),
        }
    }
}

impl XClassItem {
    /// Converts XClassItem to ClassField
    ///
    /// # Returns
    /// ClassField representation
    pub fn as_dict(&self) -> ClassField {
        let default = "";
        ClassField {
            document: vec![],
            name: self.field.clone(),
            typing: "string".to_string(),
            has_default: false,
            default: default.to_string(),
            getter: format!("get{}", self.field),
        }
    }
}
