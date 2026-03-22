use std::{any::type_name, collections::BTreeMap, fmt::Formatter};

use serde::{
    Deserialize, Deserializer,
    de::{MapAccess, Visitor},
};

use crate::{
    for_3rd::{read_map_next_extra, read_map_next_key_lowercase, read_map_next_value},
};

use super::*;

impl Default for EnumerateDescription {
    fn default() -> Self {
        Self {
            name: String::new(),
            integer: IntegerKind::default(),
            default: String::new(),
            mapping: BTreeMap::new(),
        }
    }
}

impl<'de> Deserialize<'de> for EnumerateDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Self::default())
    }
}

impl<'de> Visitor<'de> for EnumerateDescription {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = read_map_next_key_lowercase(&mut map)? {
            match key.as_str() {
                "name" => read_map_next_value(&mut map, |e| self.name = e),
                "integer" | "type" => read_map_next_value(&mut map, |e| self.integer = e),
                "default" => read_map_next_value(&mut map, |e| self.default = e),
                "mapping" => read_map_next_value(&mut map, |e| self.mapping = e),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), &key),
            }
        }
        Ok(self)
    }
}
