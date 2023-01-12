use super::*;

mod data;
mod item;

/// 生成一个类, 适用于全局配置
#[derive(Clone, Debug)]
pub struct XClassTable {
    table: CalamineTable,
    type_column: usize,
    default_column: usize,
    comment_column: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XClassData {
    /// 类名
    pub name: String,
    /// 类字段
    pub items: Vec<XClassItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XClassItem {
    pub field: String,
    pub typing: XCellTyped,
    pub default: XCellValue,
    pub document: XDocument,
}

impl XClassTable {
    fn new(table: CalamineTable) -> Self {
        Self { table, type_column: 0, default_column: 0, comment_column: 0 }
    }
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if !table.is_class() {
            return Err(XError::runtime_error("首格字段不是 Class"));
        }
        let mut out = XClassTable::new(table.clone());
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
        if out.type_column == 0 {
            return Err(XError::runtime_error("class 表未找到 type 列"));
        }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<XExportData> {
        let mut items = vec![];
        for (row, data) in self.table.rows() {
            match XClassItem::parse_cell(data, &self) {
                Ok(o) => items.push(o),
                Err(e) => {
                    log::error!("{}", e.with_y(row));
                    continue;
                }
            }
        }
        Ok(XExportData::Class(box XClassData { name: self.table.get_name(), items }))
    }
}
