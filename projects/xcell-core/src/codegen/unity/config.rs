use super::*;

static UNITY_CODEGEN_DEFAULT: LazyLock<UnityCodegen> = LazyLock::new(|| {
    let mut empty = UnityCodegen::empty();
    let config = from_str::<Value>(UNITY_CODEGEN_CONFIG).unwrap();
    empty.load_config(&config);
    empty
});

impl Default for UnityCodegen {
    fn default() -> Self {
        UNITY_CODEGEN_DEFAULT.clone()
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
        let _: Option<()> = try { self.legacy_null_null = root.get("legacy_null_null")?.as_bool()? };
    }
    pub(crate) fn empty() -> UnityCodegen {
        UnityCodegen {
            namespace: "".to_string(),
            folder_binary: "".to_string(),
            manager_name: "".to_string(),
            suffix_table: "".to_string(),
            suffix_element: "".to_string(),
            support_binary: false,
            support_clone: false,
            legacy_using: false,
            legacy_null_null: false,
        }
    }
}
