use super::*;

#[derive(Clone, Debug, Default)]
pub struct LanguageManager {
    store: BTreeMap<String, LanguageItem>,
    // data: XEnumerateData,
}

#[derive(Clone, Debug, Default)]
pub struct LanguageItem {
    key: String,
    localizations: BTreeMap<String, String>,
}

impl WorkspaceManager {
    pub fn add_language_id(&mut self, language_id: &str) -> XResult {
        match self.defines.get_enumerate(language_id) {
            None => {
                return Err(XError::runtime_error("未定义语言表"));
            }
            Some(s) => {
                s.name = "LanguageID".to_string();
            }
        }
        Ok(())
    }

    pub fn add_language_item(&mut self, language: &str, group: &str, key: &str, value: String) -> XResult {
        if key.trim().is_empty() {
            // 空 key 无效, 直接跳过不报错
            return Ok(());
        }
        let key = if group.is_empty() { key.to_string() } else { format!("{}/{}", group, key) };
        if !self.languages.store.contains_key(&key) {
            self.languages.store.insert(key.clone(), LanguageItem { key: key.clone(), localizations: Default::default() });
        }
        let item = unsafe { self.languages.store.get_mut(&key).unwrap_unchecked() };
        if item.localizations.contains_key(language) {
            return Err(XError::runtime_error(format!("语言表重复定义: {language}/{group}/{key}")));
        }
        item.localizations.insert(language.to_string(), value);
        Ok(())
    }
    /// 获取所有语言中的 key 的交集
    pub fn get_language_keys(&self) -> Vec<String> {
        self.languages.store.keys().cloned().collect()
    }
    pub fn languages(&self) -> &LanguageManager {
        let languages = self.defines.get_language_ids("LanguageID");
        todo!()
    }
}
