use super::*;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use xcell_analyzer::{UnityCodegen, WorkspaceManager, XCellHeader, XDataLine, XEnumerateData};
use xcell_types::{XError, XResult};

/// Unity enumerate code generation template
#[derive(Template)]
#[template(path = "BuildEnumerate.cs", ext = "txt", escape = "none")]
pub struct UnityEnumerate {
    /// Compiler version
    compiler_version: &'static str,
    /// Class name
    class_name: String,
    /// ID type
    id_type: &'static str,
    /// Unity codegen configuration
    config: UnityCodegen,
    /// Enumerate IDs
    enumerate_ids: Vec<EnumeratePair>,
    /// Enumerate fields
    enumerate_fields: Vec<EnumerateField>,
}

/// Enumerate field information for code generation
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

impl Display for EnumerateField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    /// Writes Unity enumerate code
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// Result of the operation
    pub(super) fn write_enumerate(&self, ws: &WorkspaceManager, table: &XEnumerateData) -> XResult<()> {
        let unity = &ws.config.unity;
        let root = &ws.config.root;
        
        let output_dir = PathBuf::from(&unity.loader.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join(format!("{}.cs", table.name));
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "// Unity generated file")?;
        writeln!(file, "")?;
        writeln!(file, "namespace {}", unity.loader.namespace)?;
        writeln!(file, "{{")?;
        writeln!(file, "    public enum {}", table.name)?;
        writeln!(file, "    {{")?;
        writeln!(file, "        // Enumerate generated from {}", table.name)?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}" )?;
        Ok(())
    }

    /// Creates Unity enumerate template data
    ///
    /// # Arguments
    /// * `table` - Enumerate data table
    ///
    /// # Returns
    /// Unity enumerate template data
    fn make_enumerate(&self, table: &XEnumerateData) -> UnityEnumerate {
        UnityEnumerate {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            class_name: table.name.clone(),
            id_type: table.typing.kind.as_csharp_type(),
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
            typing: self.typing.as_csharp_type(),
            getter: format!("Get{}", self.field_name.to_case(Case::Pascal)),
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
        EnumeratePair { key: self.key.clone(), value: data.as_csharp_value(), document: self.comment.lines() }
    }
}
