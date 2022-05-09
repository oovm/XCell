use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::TypeMetaInfo;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub path: PathBuf,
    #[serde(default, alias = "type", alias = "types")]
    pub typing: TypeMetaInfo,
}
