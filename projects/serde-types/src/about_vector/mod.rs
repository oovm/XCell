use std::{
    any::type_name,
    fmt::{Debug, Display, Formatter},
};

use serde::{
    __private::de::{Content, ContentRefDeserializer},
    de::Error,
    Deserialize, Deserializer,
};

use itertools::Itertools;

mod der;
mod ser;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OneOrManyOrNull<T>
where
    T: Default,
{
    One(T),
    Many(Vec<T>),
}

#[derive(Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OneOrMany<T> {
    inner: Vec<T>,
}

impl<T: Debug> Debug for OneOrMany<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl<T: Display> Display for OneOrMany<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.inner.iter().map(|s| s.to_string()).join(", "))
    }
}

impl<O> OneOrMany<O> {
    pub fn empty() -> Self {
        Self { inner: vec![] }
    }
    pub fn one<T>(item: T) -> Self
    where
        O: From<T>,
    {
        Self { inner: vec![O::from(item)] }
    }
    pub fn new<I, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        O: From<T>,
    {
        Self { inner: Vec::from_iter(iter.into_iter().map(O::from)) }
    }
    pub fn unwrap(self) -> Vec<O> {
        self.inner
    }
}

impl<T> OneOrManyOrNull<T>
where
    T: Default,
{
    pub fn unwrap(self) -> Vec<T> {
        match self {
            Self::One(o) => {
                vec![o]
            }
            Self::Many(o) => o,
        }
    }
}
