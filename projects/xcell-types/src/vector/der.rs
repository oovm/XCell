use super::*;
use crate::{default_deserialize, utils::push_delimiter};

impl Default for VectorDescription {
    fn default() -> Self {
        let mut delimiter = BTreeSet::default();
        push_delimiter(&mut delimiter, ";,");
        let mut suffix = BTreeSet::default();
        suffix.insert("[]".to_string());
        Self { delimiter, suffix, typing: Default::default(), default: vec![] }
    }
}

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
                "delimiter" => read_map_next_value(&mut map, |e: String| self.add_delimiter(&e)),
                "suffix" => {
                    read_map_next_value(&mut map, |e: OneOrMany<String>| e.into_iter().for_each(|s| self.add_suffix(s)))
                }
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
