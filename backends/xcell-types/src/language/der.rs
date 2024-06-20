use crate::{default_deserialize, for_3rd::read_map_next_key_lowercase};

use super::*;

default_deserialize![LanguageDescription];
impl Default for LanguageDescription {
    fn default() -> Self {
        Self { group: BTreeSet::from_iter(vec!["group".to_string()]) }
    }
}

impl<'de> Visitor<'de> for LanguageDescription {
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
                "group" => read_map_next_value(&mut map, |e: OneOrMany<String>| self.group = BTreeSet::from_iter(e.unwrap())),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), &key),
            }
        }
        Ok(self)
    }
}
