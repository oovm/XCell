use super::*;

impl Default for UnityCodegen {
    fn default() -> Self {
        Self {
            namespace: "DataTable.Generated".to_string(),
            folder_binary: "Tables/Binary".to_string(),
            manager_name: "DataTableManager".to_string(),
            suffix_table: "Table".to_string(),
            suffix_element: "Element".to_string(),
            support_binary: true,
            support_clone: true,
            legacy_using: false,
        }
    }
}

impl UnityCodegen {
    pub fn new(config: &Value) -> Self {
        let mut v = UnityCodegen::default();
        v.load_config(config);
        v
    }
    fn load_config(&mut self, root: &Value) {
        let _: Option<()> = try { self.namespace = split_namespace(root.get("namespace")?.as_str()?).join(".") };
        let _: Option<()> = try { self.folder_binary = split_file_name(root.get("folder_binary")?.as_str()?) };
        let _: Option<()> = try { self.manager_name = root.get("manager_name")?.as_str()?.to_string() };
        let _: Option<()> = try { self.suffix_table = root.get("suffix_table")?.as_str()?.to_string() };
        let _: Option<()> = try { self.suffix_element = root.get("suffix_element")?.as_str()?.to_string() };
        let _: Option<()> = try { self.support_binary = root.get("support_binary")?.as_bool()? };
        let _: Option<()> = try { self.support_clone = root.get("support_clone")?.as_bool()? };
        let _: Option<()> = try { self.legacy_using = root.get("legacy_using")?.as_bool()? };
    }
}
