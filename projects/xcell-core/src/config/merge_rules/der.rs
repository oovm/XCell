use std::any::type_name;

use serde::de::{Error, MapAccess, Visitor};

use super::*;

default_deserialize![MergeRules];

impl<'de> Visitor<'de> for MergeRules {
    type Value = Self;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(type_name::<Self>())
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
        while let Some((order, id)) = map.next_entry::<i64, MergeStep>()? {
            self.steps.insert(order, id);
        }
        Ok(self)
    }
}
