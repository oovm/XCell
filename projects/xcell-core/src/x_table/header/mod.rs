use crate::{
    config::{ProjectConfig, TableLineMode},
    CalamineTable, XDataEnumerate, XDataNumber,
};

use super::*;

impl XCellHeader {
    pub fn parse_cell(&self, row: &[DataType]) -> XResult<XCellValue> {
        match row.get(self.column) {
            Some(cell) => self.typing.parse_cell(cell),
            None => {
                todo!()
            }
        }
    }
    fn try_read_table_kind(table: &CalamineTable, project: &ProjectConfig) -> Option<XData> {
        let line = project.line.field.saturating_sub(1) as u32;
        let cell = table.get_value((line, 0))?;
        if cell.get_string()? == "enum" {
            return Some(XData::Enumerate(Box::new(XDataEnumerate::default())));
        }
        let line = project.line.typing.saturating_sub(1) as u32;
        let cell = table.get_value((line, 0))?;
        match XCellTyped::parse(cell.get_string()?, &project.typing.extra) {
            XCellTyped::Integer(_) => Some(XData::Number(Box::new(XDataNumber::default()))),
            // XCellTyped::String(_) => Some(XData::String(Box::new(XDataString::default()))),
            // 默认初始化就是 String, 就不用分配了
            XCellTyped::String(_) => None,
            _ => {
                log::error!("invalid type");
                None
            }
        }
    }
}

impl XData {
    /// 获取表格的类型, 表格类型由于第三行的第一列决定
    pub fn read_table_kind(&mut self, table: &CalamineTable, project: &ProjectConfig) {
        if let Some(s) = XCellHeader::try_read_table_kind(table, project) {
            *self = s
        }
    }
    pub fn read_table_headers(&mut self, table: &CalamineTable, project: &ProjectConfig) {
        self.read_table_kind(table, project);
        let res = match self {
            XData::Number(_) => {
                todo!()
            }
            XData::String(_) => {
                todo!()
            }
            XData::Enumerate(v) => v.read_table_headers(table, project),
        };
        match res {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

impl XDataEnumerate {
    pub fn read_table_headers(&mut self, table: &CalamineTable, project: &ProjectConfig) -> XResult<()> {
        let line = project.line.typing;
        let row = match table.rows().take(line).last() {
            Some(s) => s,
            None => return Err(XError::table_error(format!("找不到描述, 第 {} 行格式非法", line))),
        };
        for (i, data) in row.iter().enumerate() {
            if data.is_empty() {
                continue;
            }
            self.read_valid_header(table, i, project);
        }
        Ok(())
    }
    fn read_valid_header(&mut self, table: &CalamineTable, i: usize, project: &ProjectConfig) -> Option<()> {
        let line = project.line.field.saturating_sub(1) as u32;
        let field_name = table.get_value((line, i as u32))?.get_string()?;
        let mut set_id = false;
        match field_name {
            "enum" => return None,
            "comment" => {
                self.comment_column = i;
                return None;
            }
            "id" => {
                self.id_column = i;
                set_id = true;
            }
            _ => {}
        }
        let line = project.line.typing.saturating_sub(1) as u32;
        let field_type = table.get_value((line, i as u32))?.get_string()?;
        let typing = XCellTyped::parse(&field_type.to_string(), &project.typing.extra);
        match &typing {
            XCellTyped::Integer(v) if set_id => {
                self.id_type = v.kind;
                return None;
            }
            _ => {}
        }
        let (summary, details) = read_comment_details(table, i, project.line).unwrap_or_default();
        self.headers.push(XCellHeader { summary, column: i, typing, field_name: field_name.to_string(), details });
        None
    }
}

fn read_comment_details(table: &CalamineTable, i: usize, line: TableLineMode) -> Option<(String, String)> {
    let line = line.helper.saturating_sub(1) as u32;
    let comment = table.get_value((line, i as u32))?;
    let summary = comment.to_string();
    Some((summary, String::new()))
}
