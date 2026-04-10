use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use super::*;

mod der;
mod ser;

/// Unity 存储格式配置
///
/// 定义 Unity 平台的数据存储方式，支持二进制、JSON、XML 和 Protobuf 四种格式。
#[derive(Debug, Clone)]
pub enum UnityStorage {
    /// 二进制存储配置
    Binary(UnityBinaryConfig),
    /// JSON 存储配置
    Json(UnityJsonConfig),
    /// XML 存储配置
    Xml(UnityXmlConfig),
    /// Protobuf 存储配置
    Protobuf(UnityProtobufConfig),
}

impl Default for UnityStorage {
    fn default() -> Self {
        Self::Binary(UnityBinaryConfig::default())
    }
}

/// Unity 代码生成配置
///
/// 控制 Unity 平台 C# 代码生成的各项参数，包括输出路径、命名空间、
/// 管理器名称、表名后缀、存储格式等。
#[derive(Debug, Clone, Default)]
pub struct UnityCodegen {
    /// 是否启用 Unity 代码生成
    pub enable: bool,
    /// Unity 项目的工作目录，相对于配置文件
    pub project: String,
    /// 加载器输出目录，以 `project` 为根目录
    pub output: String,
    /// 生成的 C# 代码命名空间
    pub namespace: String,
    /// 生成的管理器类名
    pub manager: String,
    /// 生成的表类名后缀
    pub suffix_table: String,
    /// 生成的元素类名后缀
    pub suffix_element: String,
    /// 是否为生成的类实现 ICloneable 接口
    pub support_clone: bool,
    /// 是否使用旧版 using 语句风格
    pub legacy_using: bool,
    /// 是否使用旧版 null 合并风格
    pub legacy_null_null: bool,
    /// XLua 加载器配置
    pub xlua: UnityXluaConfig,
    /// 数据存储格式配置
    pub storage: UnityStorage,
    /// 开发时使用的数据存储格式
    pub storage_debug: Option<UnityStorage>,
}

/// Unity XLua 配置
///
/// 控制 XLua 热更新代码的生成选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityXluaConfig {
    /// 是否启用 XLua 代码生成
    pub enable: bool,
}

/// Unity XML 存储配置
///
/// 控制 XML 格式的数据输出选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityXmlConfig {
    /// 是否启用 XML 数据生成
    pub enable: bool,
    /// XML 文件输出目录
    pub output: String,
}

/// Unity JSON 存储配置
///
/// 控制 JSON 格式的数据输出选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityJsonConfig {
    /// 是否启用 JSON 数据生成
    pub enable: bool,
    /// JSON 文件输出目录
    pub output: String,
}

/// Unity Protobuf 存储配置
///
/// 控制 Protobuf 格式的数据输出选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityProtobufConfig {
    /// 是否启用 Protobuf 数据生成
    pub enable: bool,
    /// Protobuf 文件输出目录
    pub output: String,
}

impl UnityStorage {
    /// 获取存储格式的输出路径
    ///
    /// # 返回值
    /// 返回当前存储格式配置的输出目录字符串
    pub fn output_path(&self) -> &str {
        match self {
            UnityStorage::Binary(x) => x.output.as_ref(),
            UnityStorage::Json(x) => x.output.as_ref(),
            UnityStorage::Xml(x) => x.output.as_ref(),
            UnityStorage::Protobuf(x) => x.output.as_ref(),
        }
    }

    /// 获取二进制存储配置的引用
    ///
    /// 如果当前存储格式不是二进制，则返回默认配置。
    pub fn as_binary(&self) -> &UnityBinaryConfig {
        match self {
            UnityStorage::Binary(config) => config,
            _ => {
                static DEFAULT: UnityBinaryConfig = UnityBinaryConfig { enable: false, output: String::new() };
                &DEFAULT
            }
        }
    }
}

/// Unity 代码生成子项配置
///
/// 控制各类代码文件的生成开关。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityCodegenConfig {
    /// 是否生成类代码
    pub class: bool,
    /// 是否生成字典代码
    pub dictionary: bool,
    /// 是否生成枚举代码
    pub enumerate: bool,
    /// 是否生成语言表代码
    pub language: bool,
    /// 是否生成管理器代码
    pub manager: bool,
    /// 是否生成二进制数据
    pub binary: bool,
    /// 是否生成 C# 加载器代码
    pub csharp: bool,
}

/// Unity 二进制存储配置
///
/// 控制二进制格式的数据输出选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnityBinaryConfig {
    /// 是否启用二进制数据生成
    pub enable: bool,
    /// 二进制文件输出目录
    pub output: String,
}

impl UnityCodegen {
    /// 获取项目路径
    ///
    /// 根据配置中的 `project` 字段计算绝对路径。
    /// 如果 `project` 是绝对路径则直接使用，否则相对于配置文件路径拼接。
    ///
    /// # 参数
    /// * `config` - 配置文件所在目录的路径
    ///
    /// # 返回值
    /// 返回计算后的项目绝对路径
    pub fn project_path(&self, config: &Path) -> PathBuf {
        let project = Path::new(&self.project);
        if project.is_absolute() {
            return project.to_path_buf();
        }
        config.join(project)
    }

    /// 获取加载器输出路径
    ///
    /// 以项目路径为基准，拼接 `output` 字段得到加载器代码的输出路径。
    /// 该路径**必须是**相对路径。
    ///
    /// # 参数
    /// * `config` - 配置文件所在目录的路径
    ///
    /// # 返回值
    /// 返回加载器代码的输出绝对路径
    pub fn loader_path(&self, config: &Path) -> PathBuf {
        self.project_path(config).join(&self.output)
    }

    /// 获取数据文件输出路径
    ///
    /// 以项目路径为基准，拼接存储格式的 `output` 字段得到数据文件的输出路径。
    /// 如果存储格式未指定输出目录，则默认使用加载器路径。
    ///
    /// # 参数
    /// * `config` - 配置文件所在目录的路径
    ///
    /// # 返回值
    /// 返回数据文件的输出绝对路径
    pub fn data_path(&self, config: &Path) -> PathBuf {
        let data_path = self.storage.output_path();
        if data_path.is_empty() {
            return self.loader_path(config);
        }
        self.project_path(config).join(data_path)
    }

    /// 获取开发时数据文件输出路径
    ///
    /// 以项目路径为基准，拼接开发存储格式的 `output` 字段。
    /// 如果未配置开发存储格式或其输出目录为空，则默认使用数据路径。
    ///
    /// # 参数
    /// * `config` - 配置文件所在目录的路径
    ///
    /// # 返回值
    /// 返回开发时数据文件的输出绝对路径
    pub fn debug_data_path(&self, config: &Path) -> PathBuf {
        let debug_path = match self.storage_debug.as_ref() {
            None => "",
            Some(s) => s.output_path(),
        };
        if debug_path.is_empty() {
            return self.data_path(config);
        }
        self.project_path(config).join(debug_path)
    }

    /// 验证配置的合法性
    ///
    /// 检查 Unity 代码生成配置的各项参数是否合法，包括：
    /// - `output` 不能为空
    /// - `namespace` 必须是合法的 C# 命名空间
    /// - `manager` 不能为空
    ///
    /// # 返回值
    /// 如果配置合法返回 `Ok(())`，否则返回包含错误信息的 `Err`
    pub fn validate(&self) -> Result<(), String> {
        if self.output.is_empty() {
            return Err("Unity 配置的 output 不能为空".to_string());
        }
        if self.namespace.is_empty() {
            return Err("Unity 配置的 namespace 不能为空".to_string());
        }
        if !self.namespace.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_') {
            return Err(format!("Unity 配置的 namespace '{}' 包含非法字符", self.namespace));
        }
        if self.namespace.starts_with('.') || self.namespace.ends_with('.') {
            return Err(format!("Unity 配置的 namespace '{}' 不能以点号开头或结尾", self.namespace));
        }
        if self.manager.is_empty() {
            return Err("Unity 配置的 manager 不能为空".to_string());
        }
        Ok(())
    }
}
