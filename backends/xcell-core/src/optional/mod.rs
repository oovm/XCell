use std::any::type_name;

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    de::{MapAccess, Visitor},
};

use crate::{
    XResult,
    for_3rd::{read_map_next_extra, read_map_next_key_lowercase, read_map_next_value, Data},
};

use crate::{XCellTyped, XCellValue};

/// 可选类型描述，用于表示可能为空的值
#[derive(Debug, Clone, Serialize)]
pub struct OptionalDescription {
    /// 元素类型
    pub element_type: XCellTyped,
    /// 默认值
    pub default: Option<XCellValue>,
}

impl Default for OptionalDescription {
    fn default() -> Self {
        Self {
            element_type: XCellTyped::Boolean(Box::default()),
            default: None,
        }
    }
}

impl<'de> Deserialize<'de> for OptionalDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Self::default())
    }
}

impl<'de> Visitor<'de> for OptionalDescription {
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
                "default" => {
                    read_map_next_value(&mut map, |e: Option<XCellValue>| self.default = e)
                }
                _ => read_map_next_extra(&mut map, type_name::<Self>(), &key),
            }
        }
        Ok(self)
    }
}

impl From<OptionalDescription> for XCellTyped {
    fn from(value: OptionalDescription) -> Self {
        XCellTyped::Optional(Box::new(value))
    }
}

impl XCellTyped {
    /// 获取可选类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 Optional，返回 Some(&OptionalDescription)，否则返回 None
    pub fn as_optional(&self) -> Option<&OptionalDescription> {
        match self {
            XCellTyped::Optional(o) => Some(o),
            _ => None,
        }
    }

    /// 获取可变可选类型描述
    ///
    /// # 返回值
    /// 如果当前类型是 Optional，返回 Some(&mut OptionalDescription)，否则返回 None
    pub fn mut_optional(&mut self) -> Option<&mut OptionalDescription> {
        match self {
            XCellTyped::Optional(o) => Some(o),
            _ => None,
        }
    }

    /// 检查是否为可选类型
    ///
    /// # 返回值
    /// 如果当前类型是 Optional，返回 true，否则返回 false
    pub fn is_optional(&self) -> bool {
        self.as_optional().is_some()
    }
}

impl OptionalDescription {
    pub fn parse_cell(&self, cell: &Data) -> XResult<XCellValue> {
        match cell {
            Data::Empty => {
                match &self.default {
                    Some(default_value) => Ok(XCellValue::Optional(Some(Box::new(default_value.clone())))),
                    None => Ok(XCellValue::Optional(None)),
                }
            }
            _ => {
                let s = cell.to_string();
                let opt_value = xcell_parser::parse_optional(&s)?;
                match opt_value {
                    Some(value_str) => {
                        let value_cell = Data::String(value_str);
                        let value = self.element_type.parse_cell(&value_cell)?;
                        Ok(XCellValue::Optional(Some(Box::new(value))))
                    }
                    None => Ok(XCellValue::Optional(None)),
                }
            }
        }
    }
}
