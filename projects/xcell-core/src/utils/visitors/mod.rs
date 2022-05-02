pub struct StringSetVisitor {
    pub set: BTreeSet<String>,
}

impl<'de> de::Visitor<'de> for StringSetVisitor {
    type Value = ActualData;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing json data")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // unfortunately we lose some typed information
        // from errors deserializing the json string
        serde_json::from_str(v).map_err(E::custom)
    }
}
