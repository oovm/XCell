use serde::de::{
    value::{Error, MapDeserializer},
    Error as _,
};

use super::*;

impl<'de> ParsableValue<'de> {
    fn to_deserializer(self) -> ContentDeserializer<'de, Error> {
        ContentDeserializer::new(self.inner)
    }

    fn invalid_type<V>(self, v: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::invalid_type(self.unexpected(), &v))
    }
    fn custom_error<V>(self, v: impl Into<String>) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        Err(serde::de::Error::custom(v.into()))
    }

    fn unexpected(&'de self) -> Unexpected<'de> {
        match &self.inner {
            Content::Bool(b) => Unexpected::Bool(*b),
            Content::U8(n) => Unexpected::Unsigned(*n as u64),
            Content::U16(n) => Unexpected::Unsigned(*n as u64),
            Content::U32(n) => Unexpected::Unsigned(*n as u64),
            Content::U64(n) => Unexpected::Unsigned(*n),
            Content::I8(n) => Unexpected::Signed(*n as i64),
            Content::I16(n) => Unexpected::Signed(*n as i64),
            Content::I32(n) => Unexpected::Signed(*n as i64),
            Content::I64(n) => Unexpected::Signed(*n),
            Content::F32(f) => Unexpected::Float(*f as f64),
            Content::F64(f) => Unexpected::Float(*f),
            Content::Char(c) => Unexpected::Char(*c),
            Content::String(s) => Unexpected::Str(s.as_str()),
            Content::Str(s) => Unexpected::Str(s),
            Content::ByteBuf(b) => Unexpected::Bytes(b.as_slice()),
            Content::Bytes(b) => Unexpected::Bytes(b),
            Content::None | Content::Some(_) => Unexpected::Option,
            Content::Unit => Unexpected::Unit,
            Content::Newtype(_) => Unexpected::NewtypeStruct,
            Content::Seq(_) => Unexpected::Seq,
            Content::Map(_) => Unexpected::Map,
        }
    }
    fn parse_bool<V>(s: impl AsRef<str>, v: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if s.as_ref().eq_ignore_ascii_case("true") {
            v.visit_bool(true)
        }
        else if s.as_ref().eq_ignore_ascii_case("false") {
            v.visit_bool(false)
        }
        else {
            Err(Error::invalid_value(Unexpected::Str(s.as_ref()), &v))
        }
    }
    fn parse_u8<V>(s: impl AsRef<str>, v: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match u8::from_str(s.as_ref()) {
            Ok(o) => v.visit_u8(o),
            Err(_) => Err(Error::invalid_value(Unexpected::Str(s.as_ref()), &v)),
        }
    }
    fn parse_u16<V>(s: impl AsRef<str>, v: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match u16::from_str(s.as_ref()) {
            Ok(o) => v.visit_u16(o),
            Err(_) => Err(Error::invalid_value(Unexpected::Str(s.as_ref()), &v)),
        }
    }
    fn parse_u32<V>(s: impl AsRef<str>, v: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match u32::from_str(s.as_ref()) {
            Ok(o) => v.visit_u32(o),
            Err(_) => Err(Error::invalid_value(Unexpected::Str(s.as_ref()), &v)),
        }
    }
    fn parse_u64<V>(s: impl AsRef<str>, v: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match u64::from_str(s.as_ref()) {
            Ok(o) => v.visit_u64(o),
            Err(_) => Err(Error::invalid_value(Unexpected::Str(s.as_ref()), &v)),
        }
    }
    fn parse_u128<V>(s: impl AsRef<str>, v: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match u128::from_str(s.as_ref()) {
            Ok(o) => v.visit_u128(o),
            Err(_) => Err(Error::invalid_value(Unexpected::Str(s.as_ref()), &v)),
        }
    }
}

impl<'de> Deserializer<'de> for ParsableValue<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.to_deserializer().deserialize_any(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Bool(v) => visitor.visit_bool(v),
            Content::Str(v) => ParsableValue::parse_bool(v, visitor),
            Content::String(v) => ParsableValue::parse_bool(v, visitor),
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Bool(_) => {
                todo!()
            }
            Content::U8(v) => visitor.visit_u8(v),
            Content::U16(v) if v < (u8::MAX as u16) => visitor.visit_u8(v as u8),
            Content::U32(v) if v < (u8::MAX as u32) => visitor.visit_u8(v as u8),
            Content::U64(v) if v < (u8::MAX as u64) => visitor.visit_u8(v as u8),
            Content::I8(_) => {
                todo!()
            }
            Content::I16(_) => {
                todo!()
            }
            Content::I32(_) => {
                todo!()
            }
            Content::I64(_) => {
                todo!()
            }
            Content::F32(_) => {
                todo!()
            }
            Content::F64(_) => {
                todo!()
            }
            Content::String(s) => ParsableValue::parse_u8(s, visitor),
            Content::Str(s) => ParsableValue::parse_u8(s, visitor),
            Content::None => {
                todo!()
            }
            Content::Some(_) => {
                todo!()
            }
            Content::Unit => {
                todo!()
            }
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Bool(_) => {
                todo!()
            }
            Content::U8(v) => visitor.visit_u16(v as u16),
            Content::U16(v) => visitor.visit_u16(v),
            Content::U32(v) if v < (u16::MAX as u32) => visitor.visit_u16(v as u16),
            Content::U64(v) if v < (u16::MAX as u64) => visitor.visit_u16(v as u16),
            Content::I8(_) => {
                todo!()
            }
            Content::I16(_) => {
                todo!()
            }
            Content::I32(_) => {
                todo!()
            }
            Content::I64(_) => {
                todo!()
            }
            Content::F32(_) => {
                todo!()
            }
            Content::F64(_) => {
                todo!()
            }
            Content::String(s) => ParsableValue::parse_u16(s, visitor),
            Content::Str(s) => ParsableValue::parse_u16(s, visitor),
            Content::None => {
                todo!()
            }
            Content::Some(_) => {
                todo!()
            }
            Content::Unit => {
                todo!()
            }
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Bool(_) => {
                todo!()
            }
            Content::U8(v) => visitor.visit_u32(v as u32),
            Content::U16(v) => visitor.visit_u32(v as u32),
            Content::U32(v) => visitor.visit_u32(v),
            Content::U64(_) => {
                todo!()
            }
            Content::I8(_) => {
                todo!()
            }
            Content::I16(_) => {
                todo!()
            }
            Content::I32(_) => {
                todo!()
            }
            Content::I64(_) => {
                todo!()
            }
            Content::F32(_) => {
                todo!()
            }
            Content::F64(_) => {
                todo!()
            }
            Content::String(s) => ParsableValue::parse_u32(s, visitor),
            Content::Str(s) => ParsableValue::parse_u32(s, visitor),
            Content::None => {
                todo!()
            }
            Content::Some(_) => {
                todo!()
            }
            Content::Unit => {
                todo!()
            }
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Bool(_) => {
                todo!()
            }
            Content::U8(v) => visitor.visit_u64(v as u64),
            Content::U16(v) => visitor.visit_u64(v as u64),
            Content::U32(v) => visitor.visit_u64(v as u64),
            Content::U64(v) => visitor.visit_u64(v),
            Content::I8(_) => {
                todo!()
            }
            Content::I16(_) => {
                todo!()
            }
            Content::I32(_) => {
                todo!()
            }
            Content::I64(_) => {
                todo!()
            }
            Content::F32(_) => {
                todo!()
            }
            Content::F64(_) => {
                todo!()
            }
            Content::String(s) => ParsableValue::parse_u64(s, visitor),
            Content::Str(s) => ParsableValue::parse_u64(s, visitor),
            Content::None => {
                todo!()
            }
            Content::Some(_) => {
                todo!()
            }
            Content::Unit => {
                todo!()
            }
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Bool(_) => {
                todo!()
            }
            Content::U8(v) => visitor.visit_u128(v as u128),
            Content::U16(v) => visitor.visit_u128(v as u128),
            Content::U32(v) => visitor.visit_u128(v as u128),
            Content::U64(v) => visitor.visit_u128(v as u128),
            Content::I8(v) if v.is_positive() => visitor.visit_u128(v as u128),
            Content::I16(v) if v.is_positive() => visitor.visit_u128(v as u128),
            Content::I32(v) if v.is_positive() => visitor.visit_u128(v as u128),
            Content::I64(v) if v.is_positive() => visitor.visit_u128(v as u128),
            Content::F32(_) => {
                todo!()
            }
            Content::F64(_) => {
                todo!()
            }
            Content::String(s) => ParsableValue::parse_u128(s, visitor),
            Content::Str(s) => ParsableValue::parse_u128(s, visitor),
            Content::None => {
                todo!()
            }
            Content::Some(_) => {
                todo!()
            }
            Content::Unit => {
                todo!()
            }
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::String(s) => visitor.visit_str(&s),
            Content::Str(s) => visitor.visit_str(s),
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::String(s) => visitor.visit_string(s),
            Content::Str(s) => visitor.visit_string(s.to_string()),
            _ => self.invalid_type(visitor),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Seq(s) => {
                let seq = s.into_iter().map(ParsableValue::from);
                let mut seq_visitor = SeqDeserializer::new(seq);
                let value = visitor.visit_seq(&mut seq_visitor)?;
                seq_visitor.end()?;
                Ok(value)
            }
            _ => self.invalid_type(visitor),
        }
    }

    #[allow(unused_variables)]
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.inner {
            Content::Map(content) => {
                let map = content.into_iter().map(|(k, v)| (ContentDeserializer::new(k), ParsableValue::from(v)));
                let mut map_visitor = MapDeserializer::new(map);
                let value = visitor.visit_map(&mut map_visitor)?;
                map_visitor.end()?;
                Ok(value)
            }
            _ => self.invalid_type(visitor),
        }
    }

    #[allow(unused_variables)]
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.to_deserializer().deserialize_enum(name, variants, visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

impl<'de> IntoDeserializer<'de> for ParsableValue<'de> {
    type Deserializer = ParsableValue<'de>;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}
