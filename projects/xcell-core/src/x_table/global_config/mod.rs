use crate::TypeMetaInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectConfig {
    #[serde(default, alias = "type", alias = "types")]
    pub typing: TypeMetaInfo,
}
