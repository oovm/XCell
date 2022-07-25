use std::{any::type_name, collections::BTreeSet, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

use serde_types::OneOrMany;
use xcell_errors::for_3rd::{read_map_next_extra, read_map_next_value};

use crate::{default_deserialize, BooleanDescription};

default_deserialize![BooleanDescription];
impl Default for BooleanDescription {
    fn default() -> Self {
        Self {
            accept: BTreeSet::from_iter(vec!["true".to_string()]),
            reject: BTreeSet::from_iter(vec!["false".to_string()]),
            default: false,
        }
    }
}

impl<'de> Visitor<'de> for BooleanDescription {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "default" => read_map_next_value(&mut map, |e| self.default = e),
                "accept" | "true" => {
                    read_map_next_value(&mut map, |e: OneOrMany<String>| self.accept = BTreeSet::from_iter(e.unwrap()))
                }
                "reject" | "false" => {
                    read_map_next_value(&mut map, |e: OneOrMany<String>| self.accept = BTreeSet::from_iter(e.unwrap()))
                }
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
