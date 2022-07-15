use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnityCodegen {
    /// 是否要生成 unity 代码
    pub enable: bool,
    /// 生成的代码的命名空间
    pub namespace: String,
    /// 生成的二进制文件的目录
    pub folder_binary: String,
    /// 生成的管理器的名称
    pub manager_name: String,
    /// 生成的表格名的后缀
    pub suffix_table: String,
    /// 用于生成的元素名后缀
    pub suffix_element: String,
    /// 是否支持二进制序列化
    pub support_binary: bool,
    /// 支持 `IClonable` 接口
    pub support_clone: bool,
    /// 转译 `using` 语法
    pub legacy_using: bool,
    /// 转译 `??` 语法
    pub legacy_null_null: bool,
}

/// 默认的 Unity 生成配置
pub const UNITY_CODEGEN_CONFIG: &str = include_str!("UnityCodegen.toml");

static DEFAULT_CONFIG: LazyLock<UnityCodegen> = LazyLock::new(|| {
    let mut empty = UnityCodegen {
        enable: false,
        namespace: "".to_string(),
        folder_binary: "".to_string(),
        manager_name: "".to_string(),
        suffix_table: "".to_string(),
        suffix_element: "".to_string(),
        support_binary: false,
        support_clone: false,
        legacy_using: false,
        legacy_null_null: false,
    };
    let root = from_str::<Value>(UNITY_CODEGEN_CONFIG).unwrap();
    let unity = root.as_table().unwrap().get("unity").unwrap();
    empty.load_config(unity);
    // log::trace!("初始化 UNITY_CODEGEN_CONFIG\n{empty:#?}");
    empty
});

impl Default for UnityCodegen {
    fn default() -> Self {
        DEFAULT_CONFIG.clone()
    }
}

impl UnityCodegen {
    pub fn new(config: &Value) -> Self {
        let mut v = UnityCodegen::default();
        v.load_config(config);
        v
    }
    fn load_config(&mut self, root: &Value) {
        let _: Option<()> = try { self.enable = root.get("enable")?.as_bool()? };
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
}
