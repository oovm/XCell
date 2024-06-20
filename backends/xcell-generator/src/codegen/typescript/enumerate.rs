use super::*;

#[derive(Template)]
#[template(path = "BuildEnumerate.ts", ext = "txt", escape = "none")]
pub struct TypeScriptEnumerate {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// TypeScript codegen configuration
    config: TypeScriptCodegen,
    /// Enumerate IDs
    enumerate_ids: Vec<EnumeratePair>,
    /// Enumerate fields
    enumerate_fields: Vec<EnumerateField>,
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

impl TypeScriptCodegen {
    /// Writes TypeScript enumerate code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_enumerate(&self, ws: &WorkspaceManager, table: &XEnumerateData) -> XResult<()> {
        let out = match self.make_enumerate(table).render() {
            Ok(o) => o,
            Err(e) => Err(XError::runtime_error(format!("生成TypeScript枚举失败: {}", e)))?,
        };
        let mut file = self.log_typescript(ws, &table.name)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// Creates TypeScript enumerate template data
    ///
    /// # Arguments
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// TypeScript enumerate template data
    fn make_enumerate(&self, table: &XEnumerateData) -> TypeScriptEnumerate {
        TypeScriptEnumerate {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            class_name: table.name.clone(),
            enumerate_ids: table.lines.iter().map(|data| data.as_enumerate()).collect(),
            enumerate_fields: table.headers.iter().enumerate().map(|(id, data)| data.as_enumerate(&table.lines, id)).collect(),
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
