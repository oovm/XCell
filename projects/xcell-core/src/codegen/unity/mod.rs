use super::*;

mod config;
mod write_cs;

/// 默认的 Unity 生成配置
pub const UNITY_CODEGEN_CONFIG: &str = include_str!("config.toml");

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
