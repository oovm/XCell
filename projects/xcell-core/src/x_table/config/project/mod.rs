use log::trace;
use std::sync::LazyLock;

use toml::{from_str, Value};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub version: String,
    pub glob: String,
}

/// 默认的全局项目设置
pub const PROJECT_CONFIG: &str = include_str!("ProjectConfig.toml");

static DEFAULT_CONFIG: LazyLock<ProjectConfig> = LazyLock::new(|| {
    let mut empty = ProjectConfig { version: "".to_string(), glob: "".to_string() };
    let root = from_str::<Value>(PROJECT_CONFIG).unwrap();
    empty.load_config(&root);
    trace!("{empty:#?}");
    empty
});

impl Default for ProjectConfig {
    fn default() -> Self {
        DEFAULT_CONFIG.clone()
    }
}

impl FromStr for ProjectConfig {
    type Err = XError;

    fn from_str(s: &str) -> XResult<Self> {
        Ok(from_str(s)?)
    }
}

impl ProjectConfig {
    pub fn new(config: &Value) -> Self {
        let mut v = ProjectConfig::default();
        v.load_config(config);
        v
    }
    fn load_config(&mut self, root: &Value) {
        let _: Option<()> = try { self.version = root.get("version")?.as_str()?.to_string() };
        let _: Option<()> = try { self.glob = root.get("glob")?.as_str()?.trim().to_string() };
    }
}
