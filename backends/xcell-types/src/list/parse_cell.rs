use crate::utils::syntax_error;

use super::*;

impl ListDescription {
    /// 解析单元格数据，根据列表类型描述返回对应的 XCellValue
    ///
    /// # 参数
    /// * `cell` - 要解析的单元格数据
    ///
    /// # 返回值
    /// 返回解析的结果，成功时返回 XCellValue::Vector，失败时返回 XError
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        let mut out = vec![];
        let s = match cell {
            Data::Error(e) => return syntax_error(format!("未知错误 {e}")),
            _ => cell.to_string(),
        };
        if s.trim().is_empty() {
            return Ok(XCellValue::Vector(out));
        }
        for item in self.split(&s) {
            let cell = Data::String(item.to_string());
            let value = self.element_type.parse_cell(&cell)?;
            out.push(value);
        }
        if let Some(fixed) = self.fixed_length {
            if out.len() != fixed {
                return syntax_error(format!(
                    "列表长度不匹配：期望 {} 个元素，实际解析到 {} 个元素",
                    fixed,
                    out.len()
                ));
            }
        }
        Ok(XCellValue::Vector(out))
    }

    /// 使用分隔符分割字符串
    ///
    /// # 参数
    /// * `s` - 要分割的字符串
    ///
    /// # 返回值
    /// 返回分割后的字符串切片列表
    pub fn split<'i>(&self, s: &'i str) -> Vec<&'i str> {
        let mut out = Vec::new();
        for item in s.split(self.delimiter) {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            out.push(item);
        }
        out
    }
}
