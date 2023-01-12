use super::*;

impl XClassItem {
    pub fn parse_cell(data: &[DataType], cfg: &XClassTable) -> XResult<Self> {
        let mut item = XClassItem::default();
        item.field = item.try_parse_field(data)?;
        item.typing = item.try_parse_type(data, cfg)?;
        if let Ok(o) = item.parse_default(data, cfg) {
            item.default = o
        };
        item.document = XDocument::read_non_zero(data, cfg.comment_column);
        Ok(item)
    }

    fn try_parse_field(&mut self, data: &[DataType]) -> XResult<String> {
        let cell = match data.get(0).and_then(|x| x.get_string()) {
            Some("") | None => Err(XError::runtime_error("字段名为空"))?,
            Some(s) => s,
        };
        // TODO: 检查合法的类型名
        if cell.contains(' ') {
            Err(XError::runtime_error("字段名包含空格"))?
        }
        Ok(cell.to_string())
    }
    fn try_parse_type(&mut self, row: &[DataType], cfg: &XClassTable) -> XResult<XCellTyped> {
        match row.get(cfg.type_column).and_then(|v| v.get_string()) {
            Some(s) => Ok(cfg.table.parse_type(s)),
            None => Err(XError::table_error("缺失 class 类型").with_x(cfg.type_column)),
        }
    }
    fn parse_default(&mut self, row: &[DataType], cfg: &XClassTable) -> XResult<XCellValue> {
        match row.get(cfg.default_column) {
            Some(s) => Ok(self.typing.parse_cell(s).map_err(|e| e.with_x(cfg.default_column))?),
            None => Err(XError::table_error("缺失 class 值").with_x(cfg.default_column))?,
        }
    }
}
