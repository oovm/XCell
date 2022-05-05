use fs::read_to_string;
use std::{collections::BTreeSet, fs, str::FromStr};

use serde::Deserializer;

use super::*;

mod der;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableConfig {
    #[serde(default, alias = "type", alias = "types")]
    pub typing: TypeMetaInfo,
}

impl From<&ProjectConfig> for TableConfig {
    fn from(global: &ProjectConfig) -> Self {
        todo!()
    }
}

impl TableConfig {
    pub fn load_file(path: &Path) -> XResult<Self> {
        read_to_string(path)?.parse::<Self>()
    }
}

impl FromStr for TableConfig {
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
