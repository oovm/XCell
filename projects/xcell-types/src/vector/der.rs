use super::*;
use crate::{default_deserialize, utils::push_delimiter};

impl Default for VectorDescription {
    fn default() -> Self {
        let mut delimiter = BTreeSet::default();
        push_delimiter(&mut delimiter, ";,");
        Self { delimiter, suffix: Default::default(), typing: Default::default(), default: vec![] }
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
                "suffix" => read_map_next_value(&mut map, |e: OneOrMany<String>| {
                    let mut suffix = BTreeSet::default();
                    suffix.insert("[]".to_string());
                    suffix.extend(e.unwrap().into_iter());
                    for e in e.unwrap() {
                        self.add_suffix(&e)
                    }
                }),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), key),
            }
        }
        Ok(self)
    }
}
