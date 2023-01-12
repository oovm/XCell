use crate::{
    config::{ProjectConfig, TableLineMode},
    CalamineTable3, XArrayTable, XDocument, XEnumerateTable,
};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XCellHeader {
    /// 位置信息
    pub column: usize,
    /// 字段名
    pub field_name: String,
    /// 类型信息
    pub typing: XCellTyped,
    /// 短描述
    pub comment: XDocument,
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
}
