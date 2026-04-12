use super::*;
use crate::x_table::dictionary::data::{XDataLine, XDictData};
use crate::x_table::table::{ArcTableReader, XTableReader};
use calamine::{Data, DataType};
use xcell_provider::TableReader as XCellTableReader;
use xcell_core::XCellValue;

impl XLanguageTable {
    fn new(table: crate::x_table::table::ArcTableReader) -> Self {
        Self { table, language: "".to_string(), value_column: 0, group_column: 0 }
    }

    pub fn confirm(table: crate::x_table::table::ArcTableReader) -> XResult<Self> {
        let table_name = table.get_name();
        if table_name != "Language" && !table.is_language_table() {
            return Err(XError::runtime_error("不是 Language 表"));
        }
        let mut out = Self::new(table.clone());
        for header in table.headers() {
            if table.is_language_value(&header.field_name) {
                let language_name = header.field_name
                    .trim_start_matches(|c: char| !c.is_alphabetic())
                    .trim_start_matches("LanguageValue")
                    .trim_start_matches('_')
                    .trim_start_matches(|c: char| !c.is_alphabetic());
                out.language = if language_name.is_empty() { "Unknown".to_string() } else { language_name.to_string() };
                out.value_column = header.column;
            }
            if table.is_group(&header.field_name) {
                out.group_column = header.column;
            }
        }
        if out.value_column == 0 {
            for header in table.headers() {
                let field_name = header.field_name.to_lowercase();
                if field_name != "id" && field_name != "key" && field_name != "group" {
                    out.language = header.field_name.clone();
                    out.value_column = header.column;
                    break;
                }
            }
        }
        Ok(out)
    }

    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut errors = vec![];
        let mut mapping = BTreeMap::default();
        let headers: Vec<XCellHeader> = self.table.headers().collect();

        for (row, data) in self.table.rows() {
            let (group, key, value) = self.get_value(&data);
            if key.trim().is_empty() {
                continue;
            }
            if let Err(e) = ws.add_language_item(&self.language, &group, &key, value.clone()) {
                errors.push(e);
            }

            let full_key = if group.is_empty() { key.clone() } else { format!("{}/{}", group, key) };
            let mut data_line = XDataLine::default();
            data_line.key = full_key.clone();
            data_line.row = row;
            data_line.data = vec![XCellValue::String(key), XCellValue::String(group), XCellValue::String(value)];
            mapping.insert(full_key, data_line);
        }

        ws.add_dict(XDictData {
            name: "Language".to_string(),
            path: self.table.get_path(),
            headers,
            mapping,
        });

        errors
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
