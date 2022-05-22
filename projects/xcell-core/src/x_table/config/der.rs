use super::*;

use serde::de::{MapAccess, Visitor};

impl Default for BooleanMetaInfo {
    fn default() -> Self {
        Self { r#true: Default::default(), r#false: Default::default(), default: false }
    }
}

impl<'de> Deserialize<'de> for BooleanMetaInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BooleanMetaInfo::default())
    }
}

impl<'de> Visitor<'de> for BooleanMetaInfo {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("BooleanMetaInfo")
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            One(String),
            Many(Vec<String>),
            Bool(bool),
        }
        while let Some((key, value)) = map.next_entry::<&str, Helper>()? {
            match key {
                "true" => match value {
                    Helper::One(o) => {
                        self.r#true.insert(o);
                    }
                    Helper::Many(o) => self.r#true.extend(o),
                    _ => {}
                },
                "false" => match value {
                    Helper::One(o) => {
                        self.r#false.insert(o);
                    }
                    Helper::Many(o) => self.r#false.extend(o),
                    _ => {}
                },
                "default" => match value {
                    Helper::Bool(o) => self.default = o,
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(self)
    }
}
