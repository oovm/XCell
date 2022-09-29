use super::*;

mod binary;
mod enumerate;

#[derive(Serialize)]
pub struct UnityManagerWriter {
    compiler_version: &'static str,
    table_version: String,
    edit_time: String,
    config: UnityCodegen,
    tables: Vec<String>,
}

impl UnityManagerWriter {
    pub fn new(table: &MergedTable, unity: &UnityCodegen, version: &str) -> Self {
        Self {
            compiler_version: env!("CARGO_PKG_VERSION"),
            table_version: version.to_string(),
            edit_time: XCellValue::csharp_now(),
            config: unity.clone(),
            tables: table.table_names(&unity.suffix_table).into_iter().sorted().collect(),
        }
    }
}

impl UnityCodegen {
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if let Some(s) = self.unity_csharp_path(root, "test")?.parent() {
            create_dir_all(s)?
        }

        if let Some(s) = self.unity_xml_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        Ok(())
    }
    pub fn write_manager(&self, table: &MergedTable, root: &Path, version: &str) -> XResult<()> {
        let path = self.unity_manager_path(root)?;
        let ctx = Context::from_serialize(UnityManagerWriter::new(table, self, version))?;
        tera_render(include_str!("PartManager.cs.djv"), &ctx, &path, "PartManager.cs")?;
        Ok(())
    }
}

#[derive(Serialize)]
struct CSharpField {
    summary: Vec<String>,
    remarks: Vec<String>,
    typing: String,
    reader: CSharpReader,
    writer: CSharpWriter,
    name: String,
    getter: String,
    default: String,
    has_default: bool,
}
