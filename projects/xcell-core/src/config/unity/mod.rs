use super::*;

mod der;
mod ser;

#[derive(Clone, Debug, Serialize)]
pub struct UnityCodegen {
    /// 是否要生成 unity 代码
    pub enable: bool,
    /// 输出版本号
    pub version: String,
    /// unity 的工作目录, 建议使用相对路径
    ///
    /// 有 Assets 文件夹的那个
    pub project: String,
    /// 输出目录
    pub output: String,
    /// 生成的代码的命名空间
    pub namespace: String,
    /// 生成的代码的命名空间
    pub instance_name: String,
    /// 生成的管理器的名称
    pub manager_name: String,
    /// 生成的表格名的后缀
    pub suffix_table: String,
    /// 支持 `IClonable` 接口
    pub support_clone: bool,
    /// 转译 `using` 语法
    pub legacy_using: bool,
    /// 转译 `??` 语法
    pub legacy_null_null: bool,
    /// 是否支持二进制序列化
    pub binary: UnityBinaryConfig,
    /// 是否支持二进制序列化
    pub xml: UnityXmlConfig,
}

#[derive(Clone, Debug, Serialize)]
pub struct UnityBinaryConfig {
    /// 是否启用二进制生成
    pub enable: bool,
    /// 生成的二进制文件的目录
    pub output: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct UnityXmlConfig {
    /// 是否启用二进制生成
    pub enable: bool,
    /// 生成的二进制文件的目录
    pub output: String,
}

impl UnityCodegen {
    /// Unity 项目文件夹
    pub fn unity_path(&self, root: &Path) -> XResult<PathBuf> {
        let project = PathBuf::from(&self.project);
        let project = match project.is_absolute() {
            true => project,
            false => root.join(project),
        };
        Ok(project.canonicalize()?)
    }

    /// 生成二进制配置的文件夹
    pub fn unity_binary_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.unity_path(root)?.join(&self.binary.output);
        let path = dir.join(file_name).with_extension("binary");
        Ok(path)
    }
    /// 生成二进制配置的文件夹
    pub fn unity_xml_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.unity_path(root)?.join(&self.binary.output);
        let path = dir.join(file_name).with_extension("xml");
        Ok(path)
    }
    /// 生成 C# 代码的文件夹
    pub fn unity_csharp_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.unity_path(root)?.join(&self.output);
        let path = dir.join(file_name).with_extension("cs");
        Ok(path)
    }
    pub fn unity_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.unity_csharp_path(root, &self.manager_name)
    }
    pub fn unity_cs_relative(&self, file_name: &str) -> String {
        format!("{}/{}.cs", self.output, file_name)
    }
    pub fn unity_bin_relative(&self, file_name: &str) -> String {
        format!("{}/{}.binary", self.binary.output, file_name)
    }
    pub fn unity_xml_relative(&self, file_name: &str) -> String {
        format!("{}/{}.xml", self.xml.output, file_name)
    }
}
