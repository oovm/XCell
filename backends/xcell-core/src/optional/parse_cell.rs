use crate::for_3rd::Data;

use super::*;

impl OptionalDescription {
    /// 解析单元格数据，根据可选类型描述返回对应的 XCellValue
    ///
    /// # 参数
    /// * `cell` - 要解析的单元格数据
    ///
    /// # 返回值
    /// 返回解析的结果，成功时返回 XCellValue::Optional，失败时返回 XError
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        match cell {
            Data::Empty => {
                match &self.default {
                    Some(default_value) => Ok(XCellValue::Optional(Some(Box::new(default_value.clone())))),
                    None => Ok(XCellValue::Optional(None)),
                }
            }
            _ => {
                let value = self.element_type.parse_cell(cell)?;
                Ok(XCellValue::Optional(Some(Box::new(value))))
            }
        }
    }
}
