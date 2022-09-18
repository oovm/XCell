use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_types::OneOrMany;

use xcell_errors::{
    for_3rd::{read_map_next_extra, read_map_next_value},
};

use super::*;

mod der;

#[derive(Debug, Clone, Serialize)]
pub struct LanguageDescription {
    pub id: BTreeSet<String>,
    pub key: BTreeSet<String>,
    pub value: BTreeSet<String>,
    pub group: BTreeSet<String>,
}

impl From<LanguageDescription> for XCellTyped {
    fn from(value: LanguageDescription) -> Self {
        Self::Boolean(Box::new(value))
    }
}

impl LanguageDescription {
    pub fn is_id(&self, s: &str) -> bool {
        for id in self.id.iter() {
            if id.eq_ignore_ascii_case(s) {
                return true;
            }
        }
        false
    }
    pub fn is_key(&self, s: &str) -> bool {
        for key in self.key.iter() {
            if key.eq_ignore_ascii_case(s) {
                return true;
            }
        }
        false
    }
    pub fn is_value(&self, s: &str) -> bool {
        for value in self.value.iter() {
            if value.eq_ignore_ascii_case(s) {
                return true;
            }
        }
        false
    }
    pub fn is_group(&self, s: &str) -> bool {
        for group in self.group.iter() {
            if group.eq_ignore_ascii_case(s) {
                return true;
            }
        }
        false
    }
}
