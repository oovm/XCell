use crate::default_deserialize;
use serde_types::OneOrMany;

use super::*;

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
                    let mut patterns = BTreeSet::default();
                    patterns.extend(["str", "string"].iter().map(|v| v.to_string()));
                    patterns.extend(e.unwrap().into_iter());
                    self.patterns = patterns
                }),
                "default" => read_map_next_value(&mut map, |e| self.default = e),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
