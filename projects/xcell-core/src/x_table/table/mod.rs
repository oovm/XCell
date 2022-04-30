use super::*;

impl Debug for XCellTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XCellTable").field("headers", &self.headers).finish()
    }
}

impl Default for XCellTable {
    fn default() -> Self {
        Self { table: Default::default(), headers: vec![] }
    }
}

impl XCellTable {
    pub fn load_file(path: &Path) -> XResult<Self> {
        let mut xcell = Self::default();
        xcell.table = find_first_table(path)?;
        xcell.read_headers()?;
        Ok(xcell)
    }
    fn read_headers(&mut self) -> XResult<()> {
        let row = match self.table.rows().nth(0) {
            Some(s) => s,
            None => return Err(XError::table_error("找不到描述, 表第一行格式非法")),
        };
        for (i, data) in row.iter().enumerate() {
            if !data.is_empty() {
                let typing = match self.table.get_value((1, i as u32)) {
                    Some(s) => XCellType::from(s),
                    None => continue,
                };
                let field_name = match self.table.get_value((2, i as u32)) {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                self.headers.push(XCellHeader { comment: data.to_string(), column: i, typing, field_name })
            }
        }
        Ok(())
    }
}

fn find_first_table(path: &Path) -> XResult<CalamineTable> {
    let mut workbook = open_workbook_auto(path)?;
    let ranges = match workbook.worksheet_range_at(0) {
        None => return Err(XError::table_error("找不到配置表, 文件是空的, 或者表格式非法")),
        Some(s) => s?,
    };
    Ok(ranges)
}
