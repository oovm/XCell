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

#[test]
fn test() {
    let out = locale_codes::script::lookup_by_alpha("zh-hans").unwrap();
    println!("out: {:#?}", out)
}