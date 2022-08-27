use super::*;
use std::{error::Error, fmt::Display, num::ParseIntError};

impl Debug for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ParsingError").field(&self.message).finish()
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

impl serde::de::Error for ParsingError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self { message: msg.to_string(), source: None }
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(value: ParseIntError) -> Self {
        Self { message: value.to_string(), source: Some(Box::new(value)) }
    }
}
