use super::*;
use std::collections::btree_map::Values;

impl WorkspaceManager {
    pub fn add_define(&mut self, define: EnumerateDescription) -> XResult<()> {
        if self.defines.define.contains_key(define.name.as_str()) {
            return Err(XError::runtime_error(format!("重复的枚举定义: {}", define.name)));
        }
        self.defines.define.insert(define.name.to_string(), define);
        Ok(())
    }
    pub fn add_enumerate(&mut self, enumerate: XEnumerateData) {
        self.defines.enumerate.insert(enumerate.name.to_string(), enumerate);
    }
    pub fn add_class(&mut self, class: XClassData) {
        self.defines.class.insert(class.name.to_string(), class);
    }
    pub fn add_list(&mut self, list: XListData) {
        self.defines.list.insert(list.name.to_string(), list);
    }
    pub fn add_dict(&mut self, dict: XDictData) {
        self.defines.dict.insert(dict.name.to_string(), dict);
    }
}

impl WorkspaceManager {
    pub fn lists(&self) -> Values<'_, String, XListData> {
        self.defines.list.values()
    }
    pub fn dicts(&self) -> Values<'_, String, XDictData> {
        self.defines.dict.values()
    }
    pub fn classes(&self) -> Values<'_, String, XClassData> {
        self.defines.class.values()
    }
    pub fn enumerates(&self) -> Values<'_, String, XEnumerateData> {
        self.defines.enumerate.values()
    }
}
