use super::*;

impl Default for XluaCodegen {
    fn default() -> Self {
        XluaCodegen {
            enable: false,
            output: "xlua".to_string(),
            manager_name: "XCellManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "xcell".to_string(),
        }
    }
}
