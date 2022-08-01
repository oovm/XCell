use crate::default_deserialize;

use super::*;

default_deserialize![ExtraTypes];

impl<'de> Visitor<'de> for ExtraTypes {
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
                "string" => read_map_next_value(&mut map, |e: OneOrMany<String>| {
                    self.string = BTreeSet::from_iter(e.unwrap().into_iter().map(|s| s.to_ascii_lowercase()))
                }),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
