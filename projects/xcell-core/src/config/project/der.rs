use serde::de::{MapAccess, Visitor};

use super::*;

impl Default for ProjectConfig {
    fn default() -> Self {
        let xlsx = "*.xlsx";
        ProjectConfig {
            root: Default::default(),
            version: "1.0.0".to_string(),
            include: xlsx.to_string(),
            include_glob: build_glob_set(xlsx).unwrap(),
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
        deserializer.deserialize_any(ProjectConfig::default())
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
                "version" => map_next_value(&mut map, |v| self.version = v),
                "exclude" => map.next_value().map(|v| self.exclude = v).unwrap_or_default(),
                "include" => map.next_value().map(|v| self.include = v).unwrap_or_default(),

                _ => match map.next_value::<&str>() {
                    #[cfg(debug_assertions)]
                    Ok(o) => {
                        eprintln!("Unknown: ProjectConfig.{} = {:?}", key, o);
                    }
                    #[cfg(debug_assertions)]
                    Err(_) => {
                        eprintln!("Unknown: ProjectConfig.{}", key);
                    }
                    #[cfg(not(debug_assertions))]
                    _ => {}
                },
            }
        }
        Ok(self)
    }
}

pub fn map_next_value<'de, M, F, T>(dict: &mut M, mut handler: F)
where
    M: MapAccess<'de>,
    F: FnMut(T) -> (),
    T: Deserialize<'de>,
{
    match dict.next_value::<T>() {
        Ok(o) => handler(o),
        #[cfg(debug_assertions)]
        Err(e) => {
            eprintln!("e")
        }
        _ => {}
    }
}
