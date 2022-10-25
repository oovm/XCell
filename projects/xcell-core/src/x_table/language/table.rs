use super::*;

#[derive(Clone, Debug)]
pub struct XLanguageTable {
    table: CalamineTable,
    language: String,
    value_column: usize,
    group_column: usize,
}

impl XLanguageTable {
    fn new(table: CalamineTable) -> Self {
        Self { table, language: "".to_string(), value_column: 0, group_column: 0 }
    }

    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if !table.is_language_table() {
            return Err(XError::runtime_error("首格字段不是 language-id"));
        }
        let mut out = Self::new(table.clone());
        for header in table.headers() {
            if table.is_language_value(&header.field_name) {
                out.language = header.field_name.clone();
                out.value_column = header.column;
            }
            if table.is_group(&header.field_name) {
                out.group_column = header.column;
            }
        }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut out = vec![];
        for (_, data) in self.table.rows() {
            let (group, key, value) = self.get_value(data);
            if let Err(e) = ws.add_language_item(&self.language, group, key, value) {
                out.push(e);
            }
        }
        out
    }

    fn get_value<'a>(&self, row: &'a [DataType]) -> (&'a str, &'a str, String) {
        let key = row.get(0).and_then(|v| v.get_string()).unwrap_or_default();
        let group = row.get(self.group_column).and_then(|v| v.get_string()).unwrap_or_default();
        let value = row.get(self.value_column).map(|v| v.to_string()).unwrap_or_default();
        (group, key, value)
    }
}
