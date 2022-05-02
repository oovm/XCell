use fs::read_to_string;
use serde::{de, Deserializer};
use std::{fs, str::FromStr};
use std::collections::BTreeSet;
use toml::de::Error;

use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    boolean: BooleanMetaInfo,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BooleanMetaInfo {
    #[serde(deserialize_with = "BooleanMetaInfo::read_true")]
    r#true: Vec<String>,
    #[serde(deserialize_with = "BooleanMetaInfo::read_false")]
    r#false: Vec<String>,
    #[serde(deserialize_with = "BooleanMetaInfo::read_default")]
    default: bool,
}

impl BooleanMetaInfo {
    fn read_true<'de, D>(deserializer: D) -> Vec<String>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
    fn read_false<'de, D>(deserializer: D) -> Vec<String>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringListVisitor)
    }
    fn read_default<'de, D>(deserializer: D) -> Vec<String>
        where
            D: Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer) {

        }

        let s: bool = Deserialize::deserialize(deserializer)?;



        serde_json::from_str(s).map_err(de::Error::custom)
        deserializer.deserialize_bool()

        todo!()
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
