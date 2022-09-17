use std::collections::BTreeMap;

pub struct LanguageManager {

}

impl Default for LanguageManager {
    fn default() -> Self {
        Self {

        }
    }
}

pub struct LanguageItem {
    group: String,
    key: String,
}
