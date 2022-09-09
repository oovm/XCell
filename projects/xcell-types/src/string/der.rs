use serde_types::OneOrMany;

use crate::default_deserialize;

use super::*;

impl Default for StringDescription {
    fn default() -> Self {
        let mut patterns = BTreeSet::default();
        patterns.insert("string".to_string());
        patterns.insert("str".to_string());
        Self { patterns, default: "".to_string() }
    }
}

default_deserialize![StringDescription];

impl<'de> Visitor<'de> for StringDescription {
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
                "extra" => read_map_next_value(&mut map, |e: OneOrMany<String>| {
                    e.into_iter().for_each(|s| self.add_pattern(s)) // skip fmk
                }),
                "default" => read_map_next_value(&mut map, |e| self.default = e),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
