use super::*;

pub mod table;
pub mod id;
pub mod manager;
pub mod item;


impl CalamineTable {
    #[inline]
    pub fn is_language_id(&self, name: &str) -> bool {
        self.config.typing.language.is_key(name)
    }
    #[inline]
    pub fn is_language_key(&self, name: &str) -> bool {
        self.config.typing.language.is_key(name)
    }
    pub fn is_language_value(&self, name: &str) -> bool {
        self.config.typing.language.is_value(name)
    }
    #[inline]
    pub fn is_group(&self, name: &str) -> bool {
        self.config.typing.language.is_group(name)
    }
}