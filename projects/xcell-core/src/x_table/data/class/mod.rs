use log::log;

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
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path) {
        for (line, row) in table.rows().enumerate().skip(3) {
            match XClassItem::parse(row, line) {
                Ok(o) => self.items.push(o),
                Err(e) => {
                    log::error!("{}", e.with_path(path));
                }
            }
        }
    }
    pub fn link_enumerate(&mut self) {}
}

impl Default for XClassItem {
    fn default() -> Self {
        Self {
            field: "".to_string(),
            r#type: Default::default(),
            default: Default::default(),
            comment: "".to_string(),
            detail: "".to_string(),
        }
    }
}

impl XClassItem {
    fn parse(row: &[DataType], line: usize) -> XResult<Self> {
        let mut item = XClassItem {
            field: "".to_string(),
            r#type: Default::default(),
            default: Default::default(),
            comment: "".to_string(),
            detail: "".to_string(),
        };
        item.parse_field(row, line)?;
        item.parse_type(row, line)?;
        item.parse_default(row, line)?;
        item.parse_comment(row);
        Ok(item)
    }

    fn parse_field(&mut self, row: &[DataType], line: usize) -> XResult {
        match row.get(0).and_then(|v| v.get_string()) {
            None => {
                todo!()
            }
            Some(s) => self.field = s.to_string(),
        }
        Ok(())
    }
    fn parse_type(&mut self, row: &[DataType], line: usize) -> XResult {
        match row.get(0).and_then(|v| v.get_string()) {
            None => {
                todo!()
            }
            Some(s) => self.field = s.to_string(),
        }
        Ok(())
    }
    fn parse_default(&mut self, row: &[DataType], line: usize) -> XResult {
        match row.get(0).and_then(|v| v.get_string()) {
            None => {
                todo!()
            }
            Some(s) => self.field = s.to_string(),
        }
        Ok(())
    }
    fn parse_comment(&mut self, row: &[DataType]) {
        self.try_parse_comment(row);
    }
    fn try_parse_comment(&mut self, row: &[DataType]) -> Option<()> {
        let cell = row.get(3)?;
        self.comment = cell.get_string()?.to_string();
        None
    }
}
