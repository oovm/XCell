use super::*;

use crate::default_deserialize;

default_deserialize![LanguageDescription];
impl Default for LanguageDescription {
    fn default() -> Self {
        Self {
            group: BTreeSet::from_iter(vec!["true".to_string()]),
            reject: BTreeSet::from_iter(vec!["false".to_string()]),
            default: false,
        }
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
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "default" => read_map_next_value(&mut map, |e| self.default = e),
                "accept" | "true" => {
                    read_map_next_value(&mut map, |e: OneOrMany<String>| self.group = BTreeSet::from_iter(e.unwrap()))
                }
                "reject" | "false" => {
                    read_map_next_value(&mut map, |e: OneOrMany<String>| self.reject = BTreeSet::from_iter(e.unwrap()))
                }
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
