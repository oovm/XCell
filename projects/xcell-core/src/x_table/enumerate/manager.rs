use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnumerateManager {
    define: BTreeMap<String, EnumerateDescription>,
    data: BTreeMap<String, XEnumerateData>,
}

impl EnumerateManager {
    pub fn insert(&mut self, define: EnumerateDescription, data: XEnumerateData) -> XResult<()> {
        if self.define.contains_key(define.name.as_str()) {
            return Err(XError::runtime_error(format!("重复的枚举定义: {}", define.name)));
        }
        self.data.insert(data.name.to_string(), data);
        self.define.insert(define.name.to_string(), define);
        Ok(())
    }
    pub fn get(&self, name: &str) -> Option<&EnumerateDescription> {
        self.define.get(name)
    }
}
