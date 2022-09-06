use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XClassItem {
    pub field: String,
    pub r#type: XCellTyped,
    pub default: XCellValue,
    pub comment: String,
    pub detail: String,
}

impl XDataClass {
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path) -> XResult<()> {
        todo!()
    }

    pub fn link_enumerate(&mut self) {}
}
