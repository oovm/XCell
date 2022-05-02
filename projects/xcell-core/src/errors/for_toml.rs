use crate::XError;

impl From<toml::de::Error> for XError {
    fn from(e: toml::de::Error) -> Self {
        todo!()
    }
}
