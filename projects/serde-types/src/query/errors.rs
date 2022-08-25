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
