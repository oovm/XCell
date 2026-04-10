use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use oak_json::language::JsonValue;
use oak_toml::from_str;
use oak_json::to_string;

use xcell_core::TypeMetaInfo;

use super::*;
use crate::{
    cocos::CocosCodegen,
    codegen::{json::JsonCodegen, sql::SqlCodegen, typescript::TypeScriptCodegen, xlua::XluaCodegen},
    merge_rules::MergeRules,
    table::TableLineMode,
    unity::{UnityCodegen, UnityStorage, UnityXluaConfig},
};

mod der;
mod ser;

/// 导出条件结构，用于控制表格的导出行为。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCondition {
    /// 表名模式
    pub table_pattern: String,
    /// 导出目标 (server, client, both)
    pub target: String,
}

/// 生成器配置枚举
#[derive(Debug, Clone, Serialize)]
pub enum Generator {
    /// Unity 生成器
    Unity(UnityCodegen),
    /// Cocos 生成器
    Cocos(CocosCodegen),
    /// XLua 生成器
    Xlua(XluaCodegen),
    /// SQL 生成器
    Sql(SqlCodegen),
    /// JSON 生成器
    Json(JsonCodegen),
    /// TypeScript 生成器
    TypeScript(TypeScriptCodegen),
}

/// 项目配置结构，用于存储项目的全局配置信息。
/// 
/// # 配置格式
/// 
/// ## 新格式（推荐）
/// ```toml
/// [[generators]]
/// type = "Unity"
/// enable = true
/// project = "../"
/// output = "Assets/Scripts/DataTable/Generated"
/// 
/// [[generators]]
/// type = "Cocos"
/// enable = true
/// project = "../"
/// output = "assets/scripts/dataTable/generated"
/// ```
/// 
/// 新格式支持定义多个同一类型的生成器实例，例如：
/// ```toml
/// [[generators]]
/// type = "Unity"
/// 
/// [generators.unity]
/// enable = true
/// project = "../unity-project-1"
/// output = "Assets/Scripts/DataTable/Generated"
/// 
/// [[generators]]
/// type = "Unity"
/// 
/// [generators.unity]
/// enable = true
/// project = "../unity-project-2"
/// output = "Assets/Scripts/DataTable/Generated"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    #[serde(skip)]
    pub root: PathBuf,
    /// 当前版本号
    pub version: String,
    /// 包含的 excel 路径, 优先级最高
    pub include: String,
    /// 排除的 excel 模式, 优先级低于 include
    pub exclude: String,
    /// 行列排序模式
    #[serde(default)]
    pub line: TableLineMode,
    /// 类型解析模式
    #[serde(default)]
    pub typing: TypeMetaInfo,
    /// 合表模式
    #[serde(default)]
    pub merge: MergeRules,
    /// 生成器列表（新格式）
    #[serde(default)]
    pub generators: Vec<Generator>,
    /// 导出条件
    #[serde(default)]
    pub export_conditions: Vec<ExportCondition>,
}

impl ProjectConfig {
    /// 创建一个新的项目配置实例。
    ///
    /// # Parameters
    /// - `root`: 项目根目录路径
    ///
    /// # Returns
    /// - 项目配置实例
    pub fn new(root: &Path) -> Self {
        // 尝试从项目根目录读取 ProjectSettings.toml 文件
        let settings_path = root.join("ProjectSettings.toml");
        
        if settings_path.exists() {
            // 如果文件存在，从文件中加载配置
            if let Ok(content) = std::fs::read_to_string(&settings_path) {
                if let Ok(config) = from_str::<Self>(&content) {
                    let config = Self { root: root.to_path_buf(), ..config };
                    return config;
                }
            }
        } else {
            // 如果文件不存在，创建一个默认的配置文件
            let basic: Self = from_str(PROJECT_CONFIG).unwrap();
            if let Ok(config_str) = to_string(&basic) {
                if std::fs::write(&settings_path, config_str).is_ok() {
                    println!("Created ProjectSettings.toml with default configuration");
                }
            }
        }
        
        // 使用默认配置
        let basic: Self = from_str(PROJECT_CONFIG).unwrap();
        Self { root: root.to_path_buf(), ..basic }
    }
}
