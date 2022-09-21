use crate::XEnumerateTable;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XLanguageID {
    wrap: XEnumerateTable,
}

impl XLanguageID {
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if table.is_language_id() {
            return Err(XError::runtime_error("首格类型不是 LanguageID"));
        }
        Ok(Self { wrap: XEnumerateTable::force_confirm(table) })
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<XExportData> {
        Ok(XExportData::Internal)
    }
}
