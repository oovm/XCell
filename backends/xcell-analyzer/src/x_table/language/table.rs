use super::*;
use crate::x_table::table::{ArcTableReader, TableReader, WrappedTableReader};
use calamine::{Data, DataType};
use xcell_provider::TableReader as XCellTableReader;

impl XLanguageTable {
    fn new(table: crate::x_table::table::ArcTableReader) -> Self {
        Self { table, language: "".to_string(), value_column: 0, group_column: 0 }
    }

    pub fn confirm(table: crate::x_table::table::ArcTableReader) -> XResult<Self> {
        if !crate::x_table::table::TableReader::is_language_table(&table) {
            return Err(XError::runtime_error("首格字段不是 language-id"));
        }
        let mut out = Self::new(table.clone());
        for header in table.headers() {
            if crate::x_table::table::TableReader::is_language_value(&table, &header.field_name) {
                out.language = header.typing.as_csharp_type();
                out.value_column = header.column;
            }
            if crate::x_table::table::TableReader::is_group(&table, &header.field_name) {
                out.group_column = header.column;
            }
        }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut out = vec![];
        for (_, data) in self.table.rows() {
            let (group, key, value) = self.get_value(&data);
            if let Err(e) = ws.add_language_item(&self.language, &group, &key, value) {
                out.push(e);
            }
        }
        out
    }

    fn get_value(&self, row: &[Data]) -> (String, String, String) {
        let key = row.get(0).and_then(|v| v.as_string()).unwrap_or_default();
        let group = match self.group_column {
            0 => "".to_string(),
            i => row.get(i).and_then(|v| v.as_string()).unwrap_or_default(),
        };
        let value = row.get(self.value_column).map(|v| v.to_string()).unwrap_or_default();
        (group, key, value)
    }
}
