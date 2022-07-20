use serde::{de::Visitor, Deserializer};
use serde::de::MapAccess;

use super::*;

default_deserialize![BooleanDescription];
impl Default for BooleanDescription {
    fn default() -> Self {
        Self {
            accept: BTreeSet::from_iter(vec!["true".to_string()]),
            reject: BTreeSet::from_iter(vec!["false".to_string()]),
            default: false,
        }
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
        map.next_value::<OneOrMany>()


        while let Some((key, value)) = map.next_entry::<&str, OneOrMany>()? {
            match key {
                "true" => match value {
                    OneOrMany::One(o) => {
                        self.accept.insert(o);
                    }
                    OneOrMany::Many(o) => self.accept.extend(o),
                    _ => {}
                },
                "false" => match value {
                    OneOrMany::One(o) => {
                        self.reject.insert(o);
                    }
                    OneOrMany::Many(o) => self.reject.extend(o),
                    _ => {}
                },
                "default" => match value {
                    OneOrMany::Bool(o) => self.default = o,
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(self)
    }
}
