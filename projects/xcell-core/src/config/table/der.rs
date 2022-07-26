use super::*;

impl Default for LineMode {
    fn default() -> Self {
        Self { typing: 1, field: 2, helper: 3, data: 4 }
    }
}

impl<'de> Visitor<'de> for LineMode {
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
                "typing" | "type" => read_map_next_value(&mut map, |v| self.typing = v),
                "field" => read_map_next_value(&mut map, |v| self.field = v),
                "helper" | "comment" => read_map_next_value(&mut map, |v| self.helper = v),
                "data" => read_map_next_value(&mut map, |v| self.data = v),
                _ => read_map_next_extra(&mut map, type_name::<Self>(), &key),
            }
        }
        Ok(self)
    }
}
