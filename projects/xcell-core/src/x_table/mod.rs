use crate::*;
use std::path::Path;
pub mod typing;
use calamine::{open_workbook_auto, DataType, Reader};

type CalamineTable = calamine::Range<calamine::DataType>;

pub fn find_first_table(path: &Path) -> XResult<CalamineTable> {
    let mut workbook = open_workbook_auto(path)?;
    let ranges = match workbook.worksheet_range_at(0) {
        None => return Err(XError::table_error("找不到配置表, 文件是空的, 或者表格式非法")),
        Some(s) => s?,
    };
    Ok(ranges)
}

#[derive(Debug, Clone)]
pub struct XCellHeader {
    pub column: usize,
    pub comment: String,
    pub typing: XCellType,
    pub field_name: String,
}

#[derive(Debug, Clone)]
pub struct XCellTable {
    headers: Vec<XCellHeader>,
}

impl Default for XCellTable {
    fn default() -> Self {
        Self { headers: vec![] }
    }
}

impl XCellTable {
    pub fn load_file(path: &Path) -> XResult<Self> {
        let mut xcell = Self::default();
        let table = find_first_table(path)?;
        xcell.read_headers(&table)?;
        Ok(xcell)
    }
    fn read_headers(&mut self, table: &CalamineTable) -> XResult<()> {
        let row = match table.rows().nth(0) {
            Some(s) => s,
            None => return Err(XError::table_error("找不到描述, 表第一行格式非法")),
        };
        for (i, data) in row.iter().enumerate() {
            if !data.is_empty() {
                let typing = match table.get_value((1, i as u32)) {
                    Some(s) => XCellType::from(s),
                    None => continue,
                };
                let field_name = match table.get_value((2, i as u32)) {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                self.headers.push(XCellHeader { comment: data.to_string(), column: i, typing, field_name })
            }
        }
        Ok(())
    }
}
