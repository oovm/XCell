use super::*;

pub mod id;
pub mod manager;
pub mod table;

#[derive(Clone, Debug, Default)]
pub struct LanguageManager {
    pub(crate) store: BTreeMap<String, XLanguageData>,
}

#[derive(Clone, Debug)]
pub struct XLanguageID {
    wrap: XEnumerateTable,
}

#[derive(Clone, Debug)]
pub struct XLanguageTable {
    table: crate::x_table::table::ArcTableReader,
    language: String,
    value_column: usize,
    group_column: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XLanguageData {
    pub key: String,
    pub localizations: BTreeMap<String, String>,
}
