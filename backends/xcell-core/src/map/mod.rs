use std::any::type_name;
use std::collections::BTreeMap;

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

/// 映射类型描述，用于表示键值对集合
#[derive(Debug, Clone, Serialize)]
pub struct MapDescription {
    /// 键类型
    pub key_type: XCellTyped,
    /// 值类型
    pub value_type: XCellTyped,
    /// 键值分隔符（默认为冒号）
    pub delimiter: char,
    /// 条目分隔符（默认为逗号）
    pub entry_delimiter: char,
    /// 默认值
    pub default: BTreeMap<String, XCellValue>,
}

impl Default for MapDescription {
    fn default() -> Self {
        Self {
            key_type: XCellTyped::Boolean(Box::default()),
            value_type: XCellTyped::Boolean(Box::default()),
            delimiter: ':',
            entry_delimiter: ',',
            default: BTreeMap::new(),
        }
    }
}

impl<'de> Deserialize<'de> for MapDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Self::default())
    }
}

impl<'de> Visitor<'de> for MapDescription {
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
                "key_type" => {
                    read_map_next_value(&mut map, |e: XCellTyped| self.key_type = e)
                }
                "value_type" => {
                    read_map_next_value(&mut map, |e: XCellTyped| self.value_type = e)
                }
                "delimiter" => {
                    read_map_next_value(&mut map, |e: String| {
                        if let Some(c) = e.chars().next() {
                            self.delimiter = c;
                        }
                    })
                }
                "entry_delimiter" => {
                    read_map_next_value(&mut map, |e: String| {
                        if let Some(c) = e.chars().next() {
                            self.entry_delimiter = c;
                        }
                    })
                }
                "default" => {
                    read_map_next_value(&mut map, |e: BTreeMap<String, XCellValue>| self.default = e)
                }
                _ => read_map_next_extra(&mut map, type_name::<Self>(), &key),
            }
        }
        Ok(self)
    }
}

impl From<MapDescription> for XCellTyped {
    fn from(value: MapDescription) -> Self {
        XCellTyped::Map(Box::new(value))
    }
}

impl XCellTyped {
    /// 获取映射类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 Map，返回 Some(&MapDescription)，否则返回 None
    pub fn as_map(&self) -> Option<&MapDescription> {
        match self {
            XCellTyped::Map(m) => Some(m),
            _ => None,
        }
    }

    /// 获取可变映射类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 Map，返回 Some(&mut MapDescription)，否则返回 None
    pub fn mut_map(&mut self) -> Option<&mut MapDescription> {
        match self {
            XCellTyped::Map(m) => Some(m),
            _ => None,
        }
    }

    /// 检查是否为映射类型
    ///
    /// # 返回值
    /// 如果当前类型是 Map，返回 true，否则返回 false
    pub fn is_map(&self) -> bool {
        self.as_map().is_some()
    }
}
