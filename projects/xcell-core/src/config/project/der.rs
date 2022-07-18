use serde::de::{MapAccess, Visitor};

use super::*;

impl Default for ProjectConfig {
    fn default() -> Self {
        let xlsx = "*.xlsx";
        ProjectConfig {
            root: Default::default(),
            version: "1.0.0".to_string(),
            include: xlsx.to_string(),
            exclude: "".to_string(),
            unity: Default::default(),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Self::default())
    }
}

impl<'de> Visitor<'de> for ProjectConfig {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "需要是 ProjectConfig 对象")
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "version" => read_map_next_value(&mut map, |v| self.version = v),
                "exclude" => read_map_next_value(&mut map, |v| self.exclude = v),
                "include" => read_map_next_value(&mut map, |v| self.include = v),
                "unity" => read_map_next_value(&mut map, |v| self.unity = v),
                _ => read_map_next_extra(&mut map, "ProjectConfig", key),
            }
        }
        Ok(self)
    }
}
