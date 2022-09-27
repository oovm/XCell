use std::collections::btree_map::Values;

use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DictionaryManager {
    dict: BTreeMap<String, XDictTable>,
    list: BTreeMap<String, XListTable>,
}

impl WorkspaceManager {
    pub fn lists(&self) -> Values<'_, String, XListTable> {
        self.dictionaries.list.values()
    }
}
