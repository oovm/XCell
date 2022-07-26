use super::*;

impl Default for ProjectConfig {
    fn default() -> Self {
        let xlsx = "*.xlsx";
        ProjectConfig {
            root: Default::default(),
            version: "1.0.0".to_string(),
            include: xlsx.to_string(),
            exclude: "".to_string(),
            line: Default::default(),
            typing: Default::default(),
            unity: Default::default(),
        }
    }
}

impl<'de> Visitor<'de> for ProjectConfig {
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
                "version" => read_map_next_value(&mut map, |v| self.version = v),
                "exclude" => read_map_next_value(&mut map, |v: &str| self.exclude = v.trim().to_string()),
                "include" => read_map_next_value(&mut map, |v: &str| self.include = v.trim().to_string()),
                "typing" | "type" => read_map_next_value(&mut map, |v| self.typing = v),

                "unity" => read_map_next_value(&mut map, |v| self.unity = v),
                _ => read_map_next_extra(&mut map, "ProjectConfig", key),
            }
        }
        Ok(self)
    }
}
