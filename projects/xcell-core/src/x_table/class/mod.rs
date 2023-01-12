use xcell_types::TypeMetaInfo;

use super::*;

mod data;

/// 生成一个类, 适用于全局配置
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XClassTable {
    type_column: usize,
    default_column: usize,
    comment_column: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XClassData {
    pub items: Vec<XClassItem>,
    pub data: Vec<XCellValue>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XClassItem {
    pub field: String,
    pub typing: XCellTyped,
    pub document: XDocument,
}

impl XClassTable {
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if !table.is_class() {
            return Err(XError::runtime_error("首格字段不是 Class"));
        }
        let mut out = XClassTable::default();
        for header in table.headers() {
            if header.field_name.eq_ignore_ascii_case("type") {
                out.type_column = header.column;
                continue;
            }
            if header.field_name.eq_ignore_ascii_case("default") {
                out.default_column = header.column;
                continue;
            }
            if header.field_name.eq_ignore_ascii_case("comment") {
                out.comment_column = header.column;
                continue;
            }
        }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<XExportData> {
        Ok(XExportData::Internal)
    }
}

impl XClassItem {
    fn parse(row: &[DataType], line: usize, meta: &TypeMetaInfo) -> XResult<(Self, XCellValue)> {
        let mut item = XClassItem::default();
        item.parse_field(row, line)?;
        item.parse_type(row, line, meta)?;
        item.parse_comment(row);
        let data = item.parse_data(row, line)?;
        Ok((item, data))
    }

    fn parse_field(&mut self, row: &[DataType], line: usize) -> XResult {
        match self.try_parse_field(row) {
            Some(s) => self.field = s,
            None => Err(XError::table_error("非法的 class 字段名").with_xy(0, line))?,
        }
        Ok(())
    }
    fn try_parse_field(&mut self, row: &[DataType]) -> Option<String> {
        let cell = row.get(0)?.get_string()?;
        if cell.is_empty() {
            return None;
        }
        // TODO: 检查合法的类型名
        if cell.contains(' ') {
            return None;
        }
        Some(cell.to_string())
    }

    fn parse_type(&mut self, row: &[DataType], line: usize, meta: &TypeMetaInfo) -> XResult {
        match row.get(1).and_then(|v| v.get_string()) {
            Some(s) => self.r#type = XCellTyped::parse(s, meta),
            None => Err(XError::table_error("缺失 class 类型").with_xy(1, line))?,
        }
        Ok(())
    }
    fn parse_data(&mut self, row: &[DataType], line: usize) -> XResult<XCellValue> {
        match row.get(2) {
            Some(s) => Ok(self.r#type.parse_cell(s).map_err(|e| e.with_xy(0, line))?),
            None => Err(XError::table_error("缺失 class 值").with_xy(2, line))?,
        }
    }
    fn parse_comment(&mut self, row: &[DataType]) {
        self.try_parse_comment(row);
    }
    fn try_parse_comment(&mut self, row: &[DataType]) -> Option<()> {
        let cell = row.get(3)?;
        self.document = cell.get_string()?.to_string();
        None
    }
}
