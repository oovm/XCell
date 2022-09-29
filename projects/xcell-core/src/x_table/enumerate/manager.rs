use std::collections::btree_map::Values;
use super::*;

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
}

impl WorkspaceManager {
    pub fn lists(&self) -> Values<'_, String, XListData> {
        self.enumerates.list.values()
    }
    pub fn dicts(&self) -> Values<'_, String, XDictData> {
        self.enumerates.dict.values()
    }
}
