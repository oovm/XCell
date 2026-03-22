use std::any::type_name;

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    de::{MapAccess, Visitor},
};

use crate::{
    XResult,
    for_3rd::{Data, read_map_next_extra, read_map_next_key_lowercase, read_map_next_value},
};

use crate::{XCellTyped, XCellValue};

mod parse_cell;

/// 列表类型描述，用于表示元素集合
#[derive(Debug, Clone, Serialize)]
pub struct ListDescription {
    /// 元素类型
    pub element_type: XCellTyped,
    /// 固定长度（None 表示动态数组）
    pub fixed_length: Option<usize>,
    /// 分隔符（默认为逗号）
    pub delimiter: char,
    /// 默认值
    pub default: Vec<XCellValue>,
}

impl Default for ListDescription {
    fn default() -> Self {
        Self {
            element_type: XCellTyped::default(),
            fixed_length: None,
            delimiter: ',',
            default: vec![],
        }
    }
}

impl<'de> Deserialize<'de> for ListDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Self::default())
    }
}

impl<'de> Visitor<'de> for ListDescription {
    type Value = Self;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = read_map_next_key_lowercase(&mut map)? {
            match key.as_str() {
                "element_type" => {
                    read_map_next_value(&mut map, |e: XCellTyped| self.element_type = e)
                }
                "fixed_length" => {
                    read_map_next_value(&mut map, |e: usize| self.fixed_length = Some(e))
                }
                "delimiter" => {
                    read_map_next_value(&mut map, |e: String| {
                        if let Some(c) = e.chars().next() {
                            self.delimiter = c;
                        }
                    })
                }
                "default" => {
                    read_map_next_value(&mut map, |e: Vec<XCellValue>| self.default = e)
                }
                _ => read_map_next_extra(&mut map, type_name::<Self>(), &key),
            }
        }
        Ok(self)
    }
}

impl From<ListDescription> for XCellTyped {
    fn from(value: ListDescription) -> Self {
        XCellTyped::List(Box::new(value))
    }
}

impl XCellTyped {
    /// 获取列表类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 List，返回 Some(&ListDescription)，否则返回 None
    pub fn as_list(&self) -> Option<&ListDescription> {
        match self {
            XCellTyped::List(l) => Some(l),
            _ => None,
        }
    }

    /// 获取可变列表类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 List，返回 Some(&mut ListDescription)，否则返回 None
    pub fn mut_list(&mut self) -> Option<&mut ListDescription> {
        match self {
            XCellTyped::List(l) => Some(l),
            _ => None,
        }
    }

    /// 检查是否为列表类型
    ///
    /// # 返回值
    /// 如果当前类型是 List，返回 true，否则返回 false
    pub fn is_list(&self) -> bool {
        self.as_list().is_some()
    }
}
