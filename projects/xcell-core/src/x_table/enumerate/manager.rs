use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumerateManager {
    map: BTreeMap<String, EnumerateDescription>,
}

impl EnumerateManager {
    pub fn insert(&mut self, value: EnumerateDescription) -> XResult<()> {
        let name = value.name.clone();
        match self.map.insert(name, value) {
            Some(_) => Err(XError::runtime_error(format!("枚举类 `{}` 重复定义", name))),
            None => Ok(()),
        }
    }
}
