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

impl WorkspaceManager {
    pub fn collect_merged(&self) -> MergedTable {
        MergedTable { inner: self.file_mapping.values().cloned().collect() }
    }
}

pub struct MergedTable {
    inner: Vec<XCellTable>,
}

impl MergedTable {
    pub fn table_names(&self) -> Vec<String> {
        self.inner.iter().map(|v| v.name.to_string()).collect()
    }
}
