use fs::read_to_string;
use std::{fs, str::FromStr};
use toml::de::Error;

use super::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    boolean: BooleanMetaInfo,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BooleanMetaInfo {
    r#true: Vec<String>,
    r#false: Vec<String>,
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
