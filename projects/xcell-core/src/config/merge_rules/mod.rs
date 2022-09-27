use std::collections::BTreeMap;

use super::*;

mod der;
mod ser;

#[derive(Clone, Debug, Default, Serialize)]
pub struct MergeRules {
    pub enable: bool,
    pub steps: BTreeMap<i64, MergeStep>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MergeStep {
    pub input: String,
    pub output: String,
}

impl MergeRules {
    pub fn merge() {}
}

impl WorkspaceManager {}

pub struct MergedTable {
    inner: Vec<XExportData>,
}
