use std::fmt::Formatter;

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
};

use super::*;

impl Default for UnityCodegen {
    fn default() -> Self {
        Self {
            enable: false,
            namespace: "".to_string(),
            manager_name: "".to_string(),
            suffix_table: "".to_string(),
            suffix_element: "".to_string(),
            binary: Default::default(),
            support_clone: true,
            legacy_using: false,
            legacy_null_null: false,
        }
    }
}

impl Default for UnityConfigBinary {
    fn default() -> Self {
        Self { enable: true, output: "Tables/Binary".to_string() }
    }
}

impl Default for UnityConfigBinary {
    fn default() -> Self {
        Self { enable: false, output: "Tables/Binary".to_string() }
    }
}

impl<'de> Deserialize<'de> for UnityCodegen {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UnityCodegen::default())
    }
}

impl<'de> Visitor<'de> for UnityCodegen {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "需要是 bool 或者 UnityCodegen 对象")
    }

    fn visit_bool<E>(mut self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.enable = v;
        Ok(self)
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "enable" => map.next_value().map(|v| self.enable = v).unwrap_or_default(),
                _ => {
                    log::debug!("多余字段: {}", key);
                }
            }
        }
        Ok(self)
    }
}
