use fs::read_to_string;
use std::{collections::BTreeSet, fs, str::FromStr};

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
};
mod der;
use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    boolean: BooleanMetaInfo,
}

#[derive(Debug, Serialize)]
pub struct BooleanMetaInfo {
    r#true: BTreeSet<String>,
    r#false: BTreeSet<String>,
    default: bool,
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
