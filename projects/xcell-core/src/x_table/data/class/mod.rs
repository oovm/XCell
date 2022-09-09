use xcell_types::TypeMetaInfo;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XClassItem {
    pub field: String,
    pub r#type: XCellTyped,
    pub default: XCellValueKind,
    pub summary: String,
    pub details: String,
}

impl XDataClass {
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path, meta: &TypeMetaInfo) {
        for (line, row) in table.rows().enumerate().skip(3) {
            match XClassItem::parse(row, line, meta) {
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
            summary: "".to_string(),
            details: "".to_string(),
        }
    }
}

impl XClassItem {
    fn parse(row: &[DataType], line: usize, meta: &TypeMetaInfo) -> XResult<Self> {
        let mut item = XClassItem {
            field: "".to_string(),
            r#type: Default::default(),
            default: Default::default(),
            summary: "".to_string(),
            details: "".to_string(),
        };
        item.parse_field(row, line)?;
        item.parse_type(row, line, meta)?;
        item.parse_default(row, line)?;
        item.parse_comment(row);
        Ok(item)
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
    fn parse_default(&mut self, row: &[DataType], line: usize) -> XResult {
        if let Some(s) = row.get(2) {
            self.default = self.r#type.parse_cell(s).map_err(|e| e.with_xy(0, line))?
        }
        Ok(())
    }
    fn parse_comment(&mut self, row: &[DataType]) {
        self.try_parse_comment(row);
    }
    fn try_parse_comment(&mut self, row: &[DataType]) -> Option<()> {
        let cell = row.get(3)?;
        self.summary = cell.get_string()?.to_string();
        None
    }
}
