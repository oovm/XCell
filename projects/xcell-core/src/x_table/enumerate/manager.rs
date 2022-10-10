use super::*;
use std::collections::btree_map::Values;

impl WorkspaceManager {
    pub fn add_define(&mut self, define: EnumerateDescription) -> XResult<()> {
        if self.enumerates.define.contains_key(define.name.as_str()) {
            return Err(XError::runtime_error(format!("重复的枚举定义: {}", define.name)));
        }
        self.enumerates.define.insert(define.name.to_string(), define);
        Ok(())
    }
    pub fn add_enumerate(&mut self, enumerate: XEnumerateData) {
        self.enumerates.enumerate.insert(enumerate.name.to_string(), enumerate);
    }
    pub fn add_list(&mut self, list: XListData) {
        self.enumerates.list.insert(list.name.to_string(), list);
    }
    pub fn add_dict(&mut self, dict: XDictData) {
        self.enumerates.dict.insert(dict.name.to_string(), dict);
    }
}

impl WorkspaceManager {
    pub fn lists(&self) -> Values<'_, String, XListData> {
        self.enumerates.list.values()
    }
    pub fn dicts(&self) -> Values<'_, String, XDictData> {
        self.enumerates.dict.values()
    }
    pub fn classes(&self) -> Values<'_, String, XClassData> {
        self.enumerates.class.values()
    }
}
