use super::*;

#[derive(Clone, Debug, Default)]
pub struct LanguageManager {}

pub struct LanguageItem {
    group: String,
    key: String,
    mapping: BTreeMap<String, String>,
}
