use super::*;

impl Default for SqlCodegen {
    fn default() -> Self {
        SqlCodegen { enable: false, output: "sql".to_string(), database: "xcell".to_string() }
    }
}
