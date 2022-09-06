use super::*;
use crate::default_deserialize;
use serde::Deserializer;

default_deserialize![TypeMetaInfo];

impl<'de> Visitor<'de> for TypeMetaInfo {
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
                "boolean" | "bool" => read_map_next_value(&mut map, |v| self.boolean = v),
                "class" | "str" => read_map_next_value(&mut map, |v| self.string = v),
                "vector" | "vec" => read_map_next_value(&mut map, |v| self.vector = v),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), &key),
            }
        }
        Ok(self)
    }
}
