use crate::{
    XEnumerateTable,
    x_table::table::{ArcTableReader, TableReader},
};

use super::*;

impl XLanguageID {
    pub fn confirm(table: ArcTableReader) -> XResult<Self> {
        if !crate::x_table::table::TableReader::is_language_define(&table) {
            return Err(XError::runtime_error("首格类型不是 LanguageID"));
        }
        Ok(Self { wrap: XEnumerateTable::force_confirm(table) })
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut errors = self.wrap.perform(ws);
        if let Err(e) = ws.add_language_id(&self.wrap.enumerate_name()) {
            errors.push(e);
        }
        errors
    }
}
