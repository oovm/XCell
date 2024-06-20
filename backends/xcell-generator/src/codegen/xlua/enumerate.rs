use super::*;
use xcell_analyzer::{WorkspaceManager, XDataLine, XEnumerateData};
use xcell_types::XResult;

#[derive(Template)]
#[template(path = "BuildEnumerate.lua", ext = "txt", escape = "none")]
pub struct XluaEnumerate {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// Table name
    table_name: String,
    /// XLua codegen configuration
    config: XluaCodegen,
    /// Enumerate items
    items: Vec<EnumItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumItem {
    /// Item name
    name: String,
    /// Item value
    value: String,
    /// Item documentation
    document: Vec<String>,
}

impl XluaCodegen {
    /// Writes XLua enumerate code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_enumerate(&self, _ws: &WorkspaceManager, _table: &XEnumerateData) -> XResult<()> {
        // TODO: Implement write_enumerate
        Ok(())
    }

    /// Creates XLua enumerate template data
    ///
    /// # Arguments
    /// * `table` - Enumerate data table
    /// * `table_name` - Table name
    ///
    /// # Returns
    /// XLua enumerate template data
    fn make_enumerate(&self, table: &XEnumerateData, table_name: String) -> XluaEnumerate {
        XluaEnumerate {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            table_name,
            class_name: table.name.clone(),
            items: table.lines.iter().map(|s| s.as_enum()).collect(),
        }
    }
}

impl XDataLine {
    /// Converts XDataLine to EnumItem
    ///
    /// # Returns
    /// EnumItem representation
    pub fn as_enum(&self) -> EnumItem {
        EnumItem { name: self.key.clone(), value: self.id.to_string(), document: vec![] }
    }
}
