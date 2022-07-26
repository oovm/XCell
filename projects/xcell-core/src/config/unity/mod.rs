use super::*;

mod der;
mod ser;

#[derive(Clone, Debug, Serialize)]
pub struct UnityCodegen {
    /// 是否要生成 unity 代码
    pub enable: bool,
    /// unity 的工作目录, 建议使用相对路径
    ///
    /// 有 Assets 文件夹的那个
    pub project: String,
    /// 生成的代码的命名空间
    pub namespace: String,
    /// 生成的管理器的名称
    pub manager_name: String,
    /// 生成的表格名的后缀
    pub suffix_table: String,
    /// 用于生成的元素名后缀
    pub suffix_element: String,
    /// 是否支持二进制序列化
    pub binary: UnityBinaryConfig,
    /// 支持 `IClonable` 接口
    pub support_clone: bool,
    /// 转译 `using` 语法
    pub legacy_using: bool,
    /// 转译 `??` 语法
    pub legacy_null_null: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct UnityBinaryConfig {
    /// 是否启用二进制生成
    pub enable: bool,
    /// 生成的二进制文件的目录
    pub output: String,
}

impl UnityCodegen {
    /// Unity 项目文件夹
    pub fn unity_path(&self, root: &Path) -> PathBuf {
        root.to_path_buf()
    }
    /// 生成二进制配置的文件夹
    pub fn unity_binary_path(&self, root: &Path) -> PathBuf {
        self.unity_path(root)
    }
    /// 生成 C# 代码的文件夹
    pub fn unity_csharp_path(&self, root: &Path) -> PathBuf {
        self.unity_path(root)
    }
}
