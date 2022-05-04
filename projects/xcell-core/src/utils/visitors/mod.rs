use std::{collections::BTreeSet, fmt};

use de::{Error, SeqAccess};
use serde::de;

pub struct StringSetVisitor {
    pub default: BTreeSet<String>,
}

impl StringSetVisitor {
    pub fn new(value: &[&str]) -> Self {
        Self { default: BTreeSet::from_iter(value.iter().map(|f| f.to_string())) }
    }
}

pub struct BooleanVisitor {
    pub default: bool,
}

impl BooleanVisitor {
    pub fn new(value: bool) -> Self {
        Self { default: value }
    }
}

impl<'de> de::Visitor<'de> for BooleanVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing json data")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        return Ok(v);
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        return Ok(self.default);
    }
}
