
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XLanguageTable {
    table: CalamineTable,
    value_column: usize,
    group_column: usize,
}
impl XLanguageTable {
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if table.get_header(0).typing != XCellTyped::LanguageKey {
            return Err(XError::runtime_error("首格类型不是 LanguageKey"));
        }
        let mut value_column = 0;
        let mut group_column = 0;
        for header in table.headers() {
            if header.typing == XCellTyped::LanguageValue {
                value_column = header.column;
            }
            if table.is_group(&header.field_name) {
                group_column = header.column;
            }
        }
        Ok(XLanguageTable {
            table: table.clone(),
            value_column,
            group_column,
        })
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<()> {}
}
