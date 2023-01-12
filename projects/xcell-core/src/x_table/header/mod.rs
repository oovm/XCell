use crate::{
    config::{ProjectConfig, TableLineMode},
    CalamineTable3, XArrayTable, XComment, XEnumerateTable,
};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCellHeader {
    /// 位置信息
    pub column: usize,
    /// 短描述
    pub comment: XComment,
    /// 类型信息
    pub typing: XCellTyped,
    /// 字段名
    pub field_name: String,
    /// 是否是完整定义
    pub complete: bool,
}

impl XCellHeader {
    pub fn parse_cell(&self, row: &[DataType]) -> XResult<XCellValue> {
        match row.get(self.column) {
            Some(cell) => self.typing.parse_cell(cell),
            None => Err(XError::table_error("无法读取数据")),
        }
    }
    fn try_read_table_kind(table: &CalamineTable3, project: &ProjectConfig) -> Option<XExportData> {
        let line = project.line.field.saturating_sub(1) as u32;
        let cell = table.get_value((line, 0))?;
        match cell.get_string()? {
            "enum" => return Some(XExportData::Enumerate(Box::default())),
            "class" => return Some(XExportData::Class(Box::default())),
            _ => {}
        }
        let line = project.line.typing.saturating_sub(1) as u32;
        let cell = table.get_value((line, 0))?;
        match XCellTyped::parse(cell.get_string()?, &project.typing) {
            XCellTyped::Integer(_) => Some(XExportData::Array(Box::default())),
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

impl XExportData {
    /// 获取表格的类型, 表格类型由于第三行的第一列决定
    pub fn read_table_kind(&mut self, table: &CalamineTable3, project: &ProjectConfig) {
        if let Some(s) = XCellHeader::try_read_table_kind(table, project) {
            *self = s
        }
    }
    pub fn read_table_headers(&mut self, table: &CalamineTable3, project: &ProjectConfig) {
        self.read_table_kind(table, project);
        let res = match self {
            XExportData::Array(v) => v.read_table_headers(table, project),
            XExportData::Enumerate(v) => v.read_table_headers(table, project),
            XExportData::Class(_) => {
                return;
            }
            XExportData::Dictionary(_) => {
                todo!()
            }
            XExportData::Language(_) => {
                todo!()
            }
            XExportData::Internal => {}
        };
        match res {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

impl XArrayTable {
    pub fn read_table_headers(&mut self, table: &CalamineTable3, project: &ProjectConfig) -> XResult<()> {
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
    fn read_valid_header(&mut self, table: &CalamineTable3, i: usize, project: &ProjectConfig) -> Option<()> {
        let line = project.line.field.saturating_sub(1) as u32;
        let field_name = table.get_value((line, i as u32))?.get_string()?;
        let line = project.line.typing.saturating_sub(1) as u32;
        let field_type = table.get_value((line, i as u32))?.get_string()?;
        let typing = XCellTyped::parse(field_type, &project.typing);
        let (summary, details) = read_comment_details(table, i, project.line).unwrap_or_default();
        self.headers.push(XCellHeader {
            comment: summary,
            column: i,
            typing,
            field_name: field_name.to_string(),
            details,
            complete: true,
        });
        None
    }
}

impl XEnumerateTable {
    pub fn read_table_headers(&mut self, table: &CalamineTable3, project: &ProjectConfig) -> XResult<()> {
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
    fn read_valid_header(&mut self, table: &CalamineTable3, i: usize, project: &ProjectConfig) -> Option<()> {
        let line = project.line.field.saturating_sub(1) as u32;
        let field_name = table.get_value((line, i as u32))?.get_string()?;
        let mut set_id = false;
        match field_name {
            "enum" => return None,
            "comment" => {
                self.doc_column = i;
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
        let typing = XCellTyped::parse(field_type, &project.typing);
        match &typing {
            XCellTyped::Integer(v) if set_id => {
                self.id_type = v.kind;
            }
            _ => {}
        }
        let (summary, details) = read_comment_details(table, i, project.line).unwrap_or_default();
        self.headers.push(XCellHeader {
            comment: summary,
            column: i,
            typing,
            field_name: field_name.to_string(),
            details,
            complete: true,
        });
        None
    }
}

fn read_comment_details(table: &CalamineTable3, i: usize, line: TableLineMode) -> Option<(String, String)> {
    let line = line.helper.saturating_sub(1) as u32;
    let comment = table.get_value((line, i as u32))?;
    let summary = comment.to_string();
    Some((summary, String::new()))
}
