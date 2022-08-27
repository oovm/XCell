use super::*;
use std::{error::Error, fmt::Display, num::ParseIntError, str::ParseBoolError};

impl Debug for ParsableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ParsableError").field(&self.message).finish()
    }
}

impl Display for ParsableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ParsableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!()
    }
}

impl serde::de::Error for ParsableError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self { message: msg.to_string(), source: None }
    }
}

impl From<&str> for ParsableError {
    fn from(value: &str) -> Self {
        Self { message: value.to_string(), source: None }
    }
}

impl From<ParseIntError> for ParsableError {
    fn from(value: ParseIntError) -> Self {
        Self { message: value.to_string(), source: Some(Box::new(value)) }
    }
}

impl From<ParseBoolError> for ParsableError {
    fn from(value: ParseBoolError) -> Self {
        Self { message: value.to_string(), source: Some(Box::new(value)) }
    }
}
