use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XLanguageTable {
    table: CalamineTable,
    value_column: usize,
    group_column: usize,
}

impl XLanguageTable {
    fn new(table: CalamineTable) -> Self {
        Self {
            table,
            value_column: 0,
            group_column: 0,
        }
    }

    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if table.is_language_key() {
            return Err(XError::runtime_error("首格字段不是 language-id"));
        }
        let mut out = Self::new(table.clone());
        for header in table.headers() {
            if table.is_language_value(&header.field_name) {
                out.value_column = header.column;
            }
            if table.is_group(&header.field_name) {
                out.group_column = header.column;
            }
        }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<()> {

    }
}
