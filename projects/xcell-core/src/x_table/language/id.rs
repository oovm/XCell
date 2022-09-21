use super::*;
use crate::XLanguageTable;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XLanguageID {
    enum_column: usize,
}

impl XLanguageID {
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if table.get_header(0) != XCellTyped::LanguageID {
            return Err(XError::runtime_error("首格类型不是 LanguageID"));
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
        Ok(XLanguageID { table: table.clone(), value_column, group_column })
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<()> {}
}
