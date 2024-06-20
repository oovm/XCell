use std::collections::BTreeSet;

use crate::{XError, XErrorKind, XResult, for_3rd::Data};
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    pub fn unwrap(self) -> Vec<T> {
        match self {
            OneOrMany::One(t) => vec![t],
            OneOrMany::Many(v) => v,
        }
    }
}

impl<T> IntoIterator for OneOrMany<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.unwrap().into_iter()
    }
}

impl<'de, T> Deserialize<'de> for OneOrMany<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OneOrManyVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> Visitor<'de> for OneOrManyVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = OneOrMany<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a single value or a list of values")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(value) = seq.next_element()? {
                    vec.push(value);
                }
                Ok(OneOrMany::Many(vec))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let value = Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(OneOrMany::One(value))
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::BoolDeserializer::new(v))?))
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::I8Deserializer::new(v))?))
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::I16Deserializer::new(v))?))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::I32Deserializer::new(v))?))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::I64Deserializer::new(v))?))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::U8Deserializer::new(v))?))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::U16Deserializer::new(v))?))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::U32Deserializer::new(v))?))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::U64Deserializer::new(v))?))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::F32Deserializer::new(v))?))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::F64Deserializer::new(v))?))
            }

            fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::CharDeserializer::new(v))?))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::StrDeserializer::new(v))?))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::StringDeserializer::new(v))?))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::BytesDeserializer::new(v))?))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::BytesDeserializer::new(&v))?))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(de::value::UnitDeserializer::new())?))
            }
        }

        deserializer.deserialize_any(OneOrManyVisitor(std::marker::PhantomData))
    }
}

impl<T> Serialize for OneOrMany<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            OneOrMany::One(t) => t.serialize(serializer),
            OneOrMany::Many(v) => v.serialize(serializer),
        }
    }
}

#[macro_export]
macro_rules! default_deserialize {
    ($($t:ty),*) => {
        $(
            impl<'de> Deserialize<'de> for $t {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    deserializer.deserialize_any(Self::default())
                }
            }
        )*
    };
}

pub fn type_mismatch<T, S>(except: S, cell: &Data) -> XResult<T>
where
    S: Into<String>,
{
    let kind = XErrorKind::TypeMismatch { except: except.into(), current: cell.to_string() };
    Err(XError::new(kind))
}

pub fn contains_lowercase<I, S>(iter: I, s: &str) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    for item in iter {
        if item.as_ref().eq_ignore_ascii_case(s) {
            return true;
        }
    }
    false
}

pub fn syntax_error<T, A>(msg: A) -> XResult<T>
where
    A: Into<String>,
{
    let kind = XErrorKind::SyntaxError { message: msg.into() };
    Err(XError::new(kind))
}

pub fn push_delimiter(set: &mut BTreeSet<char>, new: &str) {
    set.extend(new.chars().filter(|c| !c.is_ascii_whitespace()))
}
