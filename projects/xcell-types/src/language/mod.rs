use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_types::OneOrMany;

use xcell_errors::for_3rd::{read_map_next_extra, read_map_next_value};

use crate::utils::contains_lowercase;

mod der;

#[derive(Debug, Clone, Serialize)]
pub struct LanguageDescription {
    pub group: BTreeSet<String>,
}

// impl From<LanguageDescription> for XCellTyped {
//     fn from(value: LanguageDescription) -> Self {
//         Self::Boolean(Box::new(value))
//     }
// }

impl LanguageDescription {
    pub fn is_id(&self, s: &str) -> bool {
        contains_lowercase(&["LanguageID"], s)
    }
    pub fn is_key(&self, s: &str) -> bool {
        contains_lowercase(&["LanguageKey"], s)
    }
    pub fn is_value(&self, s: &str) -> bool {
        contains_lowercase(&["LanguageValue"], s)
    }
    pub fn is_group(&self, s: &str) -> bool {
        contains_lowercase(&self.group, s)
    }
}
