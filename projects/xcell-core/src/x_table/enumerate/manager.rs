use super::*;

impl WorkspaceManager {
    pub fn add_define(&mut self, define: EnumerateDescription) -> XResult<()> {
        if self.enumerates.define.contains_key(define.name.as_str()) {
            return Err(XError::runtime_error(format!("重复的枚举定义: {}", define.name)));
        }
        self.enumerates.define.insert(define.name.to_string(), define);
        Ok(())
    }
}

impl EnumerateManager {
    pub fn add_define(&mut self, define: EnumerateDescription) -> XResult<()> {
        if self.define.contains_key(define.name.as_str()) {
            return Err(XError::runtime_error(format!("重复的枚举定义: {}", define.name)));
        }
        self.define.insert(define.name.to_string(), define);
        Ok(())
    }
    pub fn insert_enumerate(&mut self, data: XEnumerateData) -> XResult<()> {
        self.enumerate.insert(data.name.to_string(), data);
        Ok(())
    }
    pub fn get(&self, name: &str) -> Option<&EnumerateDescription> {
        self.define.get(name)
    }
}
