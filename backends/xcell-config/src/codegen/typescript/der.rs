use super::*;

impl Default for TypeScriptCodegen {
    fn default() -> Self {
        TypeScriptCodegen {
            enable: false,
            output: "typescript".to_string(),
            manager_name: "XCellManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "xcell".to_string(),
        }
    }
}
