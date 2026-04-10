use std::collections::BTreeMap;

use crate::utils::syntax_error;

use super::*;

impl MapDescription {
    /// 解析单元格数据，根据映射类型描述返回对应的 XCellValue
    ///
    /// # 参数
    /// * `cell` - 要解析的单元格数据
    ///
    /// # 返回值
    /// 返回解析的结果，成功时返回 XCellValue::Map，失败时返回 XError
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        let mut out = BTreeMap::new();
        let s = match cell {
            Data::Error(e) => return syntax_error(format!("未知错误 {e}")),
            _ => cell.to_string(),
        };
        if s.trim().is_empty() {
            return Ok(XCellValue::Map(out));
        }
        for entry in self.split_entries(&s) {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }
            let (key_str, value_str) = match entry.split_once(self.delimiter) {
                Some(pair) => pair,
                None => return syntax_error(format!("映射条目缺少键值分隔符 '{}': {}", self.delimiter, entry)),
            };
            let key_cell = Data::String(key_str.trim().to_string());
            let value_cell = Data::String(value_str.trim().to_string());
            let key_value = self.key_type.parse_cell(&key_cell)?;
            let value_value = self.value_type.parse_cell(&value_cell)?;
            let key_string = match &key_value {
                XCellValue::String(s) => s.clone(),
                other => other.to_string(),
            };
            out.insert(key_string, value_value);
        }
        Ok(XCellValue::Map(out))
    }

    /// 使用条目分隔符分割字符串
    ///
    /// # 参数
    /// * `s` - 要分割的字符串
    ///
    /// # 返回值
    /// 返回分割后的字符串切片列表
    pub fn split_entries<'i>(&self, s: &'i str) -> Vec<&'i str> {
        let mut out = Vec::new();
        for item in s.split(self.entry_delimiter) {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            out.push(item);
        }
        out
    }
}
