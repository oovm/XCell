use serde::{
    de::Error,
    Deserialize, Deserializer,
    __private::de::{Content, ContentRefDeserializer},
};

mod der;
mod ser;

#[derive(Debug)]
pub enum OneOrManyOrNull<T>
where
    T: Default,
{
    One(T),
    Many(Vec<T>),
}

#[derive(Debug)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<'de, T> Deserialize<'de> for OneOrMany<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let content = <Content as Deserialize>::deserialize(deserializer)?;
        if let Ok(__ok) =
            Result::map(<T as Deserialize>::deserialize(ContentRefDeserializer::<D::Error>::new(&content)), OneOrMany::One)
        {
            return Ok(__ok);
        }
        if let Ok(__ok) = Result::map(
            <Vec<T> as Deserialize>::deserialize(ContentRefDeserializer::<D::Error>::new(&content)),
            OneOrMany::Many,
        ) {
            return Ok(__ok);
        }
        Err(D::Error::custom("data did not match any variant of untagged enum OneOrMany"))
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

impl<T> OneOrMany<T> {
    pub fn unwrap(self) -> Vec<T> {
        match self {
            Self::One(o) => {
                vec![o]
            }
            Self::Many(o) => o,
        }
    }
}
