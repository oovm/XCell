use super::*;

impl CalamineTable2 {
    pub fn load(path: &Path, config: &ProjectConfig) -> XResult<Self> {
        let table = find_first_table(path)?;
        // let toml = config.get_table_config(&table)?;
        Ok(Self {
            table,
            line: config.line,
            typing: config.typing.clone(),
        })
    }
    pub fn get_header(&self, index: usize) -> XCellHeader {
        let mut complete = true;
        let field_name = match self.get_field_name(index) {
            Some(s) => { s }
            None => {
                complete = false;
                Default::default()
            }
        };
        let typing = match self.get_field_type(index) {
            Some(s) => { s }
            None => {
                complete = false;
                Default::default()
            }
        };
        let (summary, details) = self.read_comment_details(index).unwrap_or_default();
        XCellHeader {
            column: index,
            summary,
            details,
            typing,
            field_name,
            complete,
        }
    }
    fn get_field_name(&self, index: usize) -> Option<String> {
        let line = self.line.field.saturating_sub(1) as u32;
        let value = self.table.get_value((line, index as u32))?.get_string()?;
        if value.is_empty() {
            return None;
        }
        Some(value.to_string())
    }
    fn get_field_type(&self, index: usize) -> Option<XCellTyped> {
        let line = self.line.typing.saturating_sub(1) as u32;
        let value = self.table.get_value((line, index as u32))?.get_string()?;
        if value.is_empty() {
            return None;
        }
        let typing = XCellTyped::parse(value, &self.typing);
        Some(typing)
    }
    fn read_comment_details(&self, index: usize) -> Option<(String, String)> {
        let line = self.line.helper.saturating_sub(1) as u32;
        let comment = self.table.get_value((line, index as u32))?;
        let summary = comment.to_string();
        Some((summary, String::new()))
    }
}