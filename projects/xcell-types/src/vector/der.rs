use super::*;
use crate::default_deserialize;

default_deserialize![VectorDescription];

impl<'de> Visitor<'de> for VectorDescription {
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
                "delimiter" => read_map_next_value(&mut map, |e: String| {
                    self.delimiter = BTreeSet::from_iter(e.chars().filter(|c| !c.is_ascii_whitespace()))
                }),
                "suffix" => read_map_next_value(&mut map, |e: OneOrMany<String>| {
                    let mut suffix = BTreeSet::default();
                    suffix.insert("[]".to_string());
                    suffix.extend(e.unwrap().into_iter());
                    self.suffix = suffix
                }),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
