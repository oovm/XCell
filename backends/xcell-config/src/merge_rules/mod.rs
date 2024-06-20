use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::*;

mod der;
mod ser;

/// 合并规则配置
#[derive(Debug, Clone, Default, Serialize)]
pub struct MergeRules {
    pub enable: bool,
    pub steps: BTreeMap<i64, MergeStep>,
}

/// 合并步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeStep {
    pub input: String,
    pub output: String,
}

impl MergeRules {
    pub fn merge() {}
}
