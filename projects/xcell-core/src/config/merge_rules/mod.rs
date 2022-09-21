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
    inner: Vec<XTable>,
}

impl MergedTable {
    pub fn table_names(&self, suffix_table: &str) -> Vec<String> {
        self.inner
            .iter()
            .map(|v| match &v.data {
                XExportData::Array(_) => {
                    format!("{}{}", v.name, suffix_table)
                }
                XExportData::Enumerate(_) => {
                    format!("{}{}", v.name, suffix_table)
                }
                XExportData::Class(_) => v.name.to_string(),
                XExportData::Dictionary(_) => {
                    todo!()
                }
                XExportData::Language(_) => {
                    todo!()
                }
            })
            .collect()
    }
}
