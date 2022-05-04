use fs::read_to_string;
use std::{collections::BTreeSet, fs, str::FromStr};

use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};

use crate::utils::visitors::{BooleanVisitor, StringSetVisitor};

use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    boolean: BooleanMetaInfo,
}

#[derive(Debug, Serialize)]
pub struct BooleanMetaInfo {
    #[serde(deserialize_with = "BooleanMetaInfo::read_true")]
    r#true: BTreeSet<String>,
    #[serde(deserialize_with = "BooleanMetaInfo::read_false")]
    r#false: BTreeSet<String>,
    #[serde(deserialize_with = "BooleanMetaInfo::read_default")]
    default: bool,
}

impl Default for BooleanMetaInfo {
    fn default() -> Self {
        Self { r#true: Default::default(), r#false: Default::default(), default: false }
    }
}

impl<'de> Visitor<'de> for BooleanMetaInfo {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("BooleanMetaInfo")
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(k) = map.next_key::<&str>()? {
            match k {
                "true" => self.r#true = StringSetVisitor::new(&["true"]).visit_map(&mut map)?,
                "false" => self.r#false = StringSetVisitor::new(&["false"]).visit_map(&mut map)?,
                "default" => map.next_value()?,
                _ => {}
            }
        }
        Ok(self)
    }
}

impl StringSetVisitor {
    fn visit_map<'de, A>(mut self, mut map: A) -> Result<BTreeSet<String>, A::Error>
    where
        A: MapAccess<'de>,
    {
        match map.next_value::<String>() {
            Ok(o) => {
                self.default.insert(o);
            }
            Err(_) => match map.next_value::<Vec<String>>() {
                Ok(o) => {
                    self.default.extend(o.into_iter());
                }
                Err(_) => {}
            },
        }
        Ok(self.default)
    }
}

impl<'de> Deserialize<'de> for BooleanMetaInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let info = BooleanMetaInfo::default();
        deserializer.deserialize_any(info)
    }
}

impl BooleanMetaInfo {
    fn read_true<'de, D>(deserializer: D) -> Result<BTreeSet<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringSetVisitor::new(&["true"]))
    }
    fn read_false<'de, D>(deserializer: D) -> Result<BTreeSet<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringSetVisitor::new(&["false"]))
    }
    fn read_default<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BooleanVisitor::new(false))
    }
}

impl XCellMetaInfo {
    pub fn load_file(path: &Path) -> XResult<Self> {
        read_to_string(path)?.parse::<Self>()
    }
}

impl FromStr for XCellMetaInfo {
    type Err = XError;

    fn from_str(s: &str) -> XResult<Self> {
        Ok(toml::from_str(s)?)
    }
}
