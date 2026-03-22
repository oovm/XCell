use super::*;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use dejavu::Template;
use xcell_analyzer::{XCellHeader, XDataLine};

#[derive(Template)]
#[template(path = "BuildEnumerate.ts.dejavu")]
pub struct CocosEnumerateTemplate {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// ID type
    id_type: &'static str,
    /// Cocos codegen configuration
    config: CocosCodegen,
    /// Enumerate IDs
    enumerate_ids: Vec<EnumeratePair>,
    /// Class document
    class_document: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumerateField {
    /// Field documentation
    document: Vec<String>,
    /// Switch cases
    switch: Vec<EnumeratePair>,
    /// Field name
    name: String,
    /// Field type
    typing: String,
    /// Getter method
    getter: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    /// Key
    key: String,
    /// Value
    value: String,
    /// Documentation
    document: Vec<String>,
}

impl CocosCodegen {
    /// Writes Cocos enumerate code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_enumerate(&self, ws: &WorkspaceManager, table: &XEnumerateData) -> XResult<()> {
        let out = match self.make_enumerate(table).render(&dejavu_types::values::Context::new()) {
            Ok(o) => o,
            Err(e) => Err(XError::runtime_error(format!("生成Cocos枚举失败: {}", e)))?,
        };
        let mut file = self.log_typescript(ws, &table.name)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates Cocos enumerate template data
    ///
    /// # Arguments
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// Cocos enumerate template data
    fn make_enumerate(&self, table: &XEnumerateData) -> CocosEnumerateTemplate {
        CocosEnumerateTemplate {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            class_name: table.name.clone(),
            id_type: table.typing.kind.as_typescript_type(),
            enumerate_ids: table.lines.iter().map(|data| data.as_enumerate()).collect(),
            class_document: vec![],
        }
    }
}

impl XCellHeader {
    /// Converts XCellHeader to EnumerateField
    ///
    /// # Arguments
    /// * `values` - Data lines
    /// * `index` - Field index
    ///
    /// # Returns
    /// EnumerateField representation
    fn as_enumerate(&self, values: &[XDataLine], index: usize) -> EnumerateField {
        EnumerateField {
            name: self.field_name.clone(),
            typing: self.typing.as_typescript_type(),
            getter: format!("get{}", self.field_name.to_case(Case::Pascal)),
            document: self.document.lines(),
            switch: values.iter().map(|data| data.as_pair(index)).collect(),
        }
    }
}

impl XDataLine {
    /// Converts XDataLine to EnumeratePair for IDs
    ///
    /// # Returns
    /// EnumeratePair representation
    fn as_enumerate(&self) -> EnumeratePair {
        EnumeratePair { key: self.key.clone(), value: self.id.to_string(), document: self.comment.lines() }
    }

    /// Converts XDataLine to EnumeratePair for field values
    ///
    /// # Arguments
    /// * `index` - Field index
    ///
    /// # Returns
    /// EnumeratePair representation
    fn as_pair(&self, index: usize) -> EnumeratePair {
        // 枚举和字段一样长, 必定存在
        let data = self.data.get(index).unwrap();
        EnumeratePair { key: self.key.clone(), value: data.as_typescript_value(), document: self.comment.lines() }
    }
}
