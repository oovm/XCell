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
    pub group: BTreeSet<String>,
}

impl From<LanguageDescription> for XCellTyped {
    fn from(value: LanguageDescription) -> Self {
        Self::Boolean(Box::new(value))
    }
}

impl LanguageDescription {
    pub fn is_group(&self, s: &str) -> bool {
        for group in self.group.iter() {
            if group.eq_ignore_ascii_case(s) {
                return true;
            }
        }
        false
    }
}
