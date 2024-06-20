use super::*;

impl WorkspaceManager {
    pub fn add_language_id(&mut self, language_id: &str) -> XResult<()> {
        match self.defines.enumerate.remove(language_id) {
            Some(mut s) => {
                s.name = "LanguageID".to_string();
                self.add_enumerate(s);
                Ok(())
            }
            None => return Err(XError::runtime_error("未定义语言表")),
        }
    }

    pub fn add_language_item(&mut self, language: &str, group: &str, key: &str, value: String) -> XResult<()> {
        if key.trim().is_empty() {
            // 空 key 无效, 直接跳过不报错
            return Ok(());
        }
        let key = if group.is_empty() { key.to_string() } else { format!("{group}/{key}") };
        if !self.languages.store.contains_key(&key) {
            self.languages.store.insert(key.clone(), XLanguageData { key: key.clone(), localizations: Default::default() });
        }
        let item = unsafe { self.languages.store.get_mut(&key).unwrap_unchecked() };
        if item.localizations.contains_key(language) {
            return Err(XError::runtime_error(format!("语言表重复定义: {language}/{group}/{key}")));
        }
        item.localizations.insert(language.to_string(), value);
        Ok(())
    }
    /// 获取所有已定义的语言
    pub fn get_language_ids(&self) -> Vec<&str> {
        match self.defines.enumerate.get("LanguageID") {
            Some(s) => s.lines.iter().map(|s| s.key.as_str()).collect(),
            None => {
                vec![]
            }
        }
    }
    /// 获取所有语言中的 key 的交集
    pub fn get_language_keys(&self) -> Vec<&str> {
        self.languages.store.keys().map(|s| s.as_str()).collect()
    }
    pub fn languages(&self) -> Vec<XLanguageData> {
        let mut langs = BTreeMap::default();
        for id in self.get_language_ids() {
            langs.insert(id.to_string(), XLanguageData { key: id.to_string(), localizations: Default::default() });
        }
        for item in self.languages.store.values() {
            for language in langs.values_mut() {
                if let Some(s) = item.localizations.get(&language.key) {
                    language.localizations.insert(item.key.clone(), s.clone());
                }
            }
        }
        langs.into_values().collect()
    }
}
