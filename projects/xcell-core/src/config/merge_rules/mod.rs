use std::collections::BTreeMap;

use super::*;

mod der;
mod ser;

#[derive(Clone, Debug, Serialize)]
pub struct MergeRules {
    pub steps: BTreeMap<i32, MergeStep>,
}

pub struct MergeStep {
    pub input: String,
    pub output: String,
}

impl MergeRules {
    pub fn merge() {}
}

impl WorkspaceManager {
    pub fn collect_merged(&self) -> TableMerged {
        TableMerged { inner: self.file_mapping.values().cloned().collect() }
    }
}

pub struct TableMerged {
    inner: Vec<XCellTable>,
}

impl TableMerged {
    pub fn table_names(&self) -> Vec<String> {
        self.inner.iter().map(|v| v.name.to_string()).collect()
    }
}
