use super::*;

impl TableConfig {
    pub fn load_file(path: &Path) -> XResult<Self> {
        read_to_string(path)?.parse::<Self>()
    }
}

impl FromStr for TableConfig {
    type Err = XError;

    fn from_str(s: &str) -> XResult<Self> {
        Ok(toml::from_str(s)?)
    }
}
