use super::*;

impl Debug for XCellTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XCellTable")
            .field("path", &self.path)
            .field("config", &self.config)
            .field("headers", &self.headers)
            .field("errors", &self.errors)
            .finish()
    }
}

impl Default for XCellTable {
    fn default() -> Self {
        Self {
            path: Default::default(),
            headers: vec![],
            config: Default::default(),
            sum_excel: 0,
            sum_config: 0,
            errors: vec![],
        }
    }
}
