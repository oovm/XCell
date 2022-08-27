use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use serde::{
    __private::de::{Content, ContentDeserializer},
    de::{
        value::{MapDeserializer, SeqDeserializer},
        DeserializeSeed, MapAccess, Visitor,
    },
    Deserialize, Deserializer,
};

mod der;
mod errors;

pub struct ParsingValue<'de> {
    inner: Content<'de>,
}

pub struct ParsingError {
    pub message: String,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl Default for ParsingValue<'_> {
    fn default() -> Self {
        Self { inner: Content::Map(vec![]) }
    }
}

impl Debug for ParsingValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<'de> ParsingValue<'de> {
    pub fn text(s: &'de str) -> Self {
        Self { inner: Content::Str(s) }
    }
    pub fn get(&self, key: &str) -> Option<&Content<'de>> {
        match &self.inner {
            Content::Map(map) => {
                for (k, v) in map.iter().rev() {
                    if k.as_str()? == key {
                        return Some(v);
                    }
                }
            }
            _ => unreachable!(),
        }
        None
    }
    pub fn insert(&mut self, key: &'de str, value: Content<'de>) {
        match &mut self.inner {
            Content::Map(map) => {
                map.push((Content::Str(key), value));
            }
            _ => {}
        }
    }
    pub fn insert_header(&mut self) {}
}
