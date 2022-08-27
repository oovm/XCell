use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};

use serde::{
    __private::de::{Content, ContentDeserializer},
    de::{DeserializeSeed, MapAccess, Visitor},
    Deserialize, Deserializer,
};

mod der;
mod errors;

pub struct ParsableValue<'de> {
    inner: Content<'de>,
}

pub struct ParsableError {
    pub message: String,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl Default for ParsableValue<'_> {
    fn default() -> Self {
        Self { inner: Content::Map(vec![]) }
    }
}

impl Debug for ParsableValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<'de> From<Content<'de>> for ParsableValue<'de> {
    fn from(value: Content<'de>) -> Self {
        Self { inner: value }
    }
}

impl<'de> ParsableValue<'de> {
    pub fn new(v: Content<'de>) -> Self {
        Self { inner: v }
    }
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
        if let Content::Map(map) = &mut self.inner {
            map.push((Content::Str(key), value));
        }
    }
    pub fn insert_header(&mut self) {}
}
