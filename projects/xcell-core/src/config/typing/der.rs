use std::{any::type_name, fmt::Formatter};

use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};

use super::*;

impl Default for BooleanDescription {
    fn default() -> Self {
        Self { accept: Default::default(), reject: Default::default(), default: false }
    }
}

impl<'de> Deserialize<'de> for BooleanDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Self::default())
    }
}

impl<'de> Visitor<'de> for BooleanDescription {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
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
                        self.accept.insert(o);
                    }
                    Helper::Many(o) => self.accept.extend(o),
                    _ => {}
                },
                "false" => match value {
                    Helper::One(o) => {
                        self.reject.insert(o);
                    }
                    Helper::Many(o) => self.reject.extend(o),
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
