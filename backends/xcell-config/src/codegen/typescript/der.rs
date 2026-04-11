use super::*;

impl Default for TypeScriptCodegen {
    fn default() -> Self {
        TypeScriptCodegen {
            enable: false,
            project: "..".to_string(),
            output: "typescript".to_string(),
            storage: "".to_string(),
            storage_type: "json".to_string(),
            manager_name: "XCellManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "xcell".to_string(),
            loader_template: "".to_string(),
        }
    }
}
