use super::*;

#[derive(Template)]
#[template(path = "BuildManager.cs.djv", ext = "txt", escape = "none")]
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
            tables: vec![],
        }
    }
}
