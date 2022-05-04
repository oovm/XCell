use fs::read_to_string;
use std::{collections::BTreeSet, fs, str::FromStr};

use serde::Deserializer;

use super::*;

mod der;

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    pub boolean: BooleanMetaInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct BooleanMetaInfo {
    pub r#true: BTreeSet<String>,
    pub r#false: BTreeSet<String>,
    pub default: bool,
}
