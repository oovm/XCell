use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use oak_json::language::JsonValue;
use oak_toml::from_str;
use oak_json::to_string;

use xcell_core::TypeMetaInfo;

use super::*;
use crate::{
    cocos::{CocosCodegen, CocosJsonConfig, CocosStorage},
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
    #[serde(default = "default_version")]
    pub version: String,
    /// 数据路径
    #[serde(default = "default_project")]
    pub project: String,
    /// 包含的 excel 路径, 优先级最高
    #[serde(default = "default_include")]
    pub include: String,
    /// 排除的 excel 模式, 优先级低于 include
    #[serde(default = "default_exclude")]
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
    #[serde(default = "default_generators")]
    pub generators: Vec<Generator>,
    /// 导出条件
    #[serde(default)]
    pub export_conditions: Vec<ExportCondition>,
}

fn default_version() -> String {
    "0.0.0".to_string()
}

fn default_project() -> String {
    "sheets".to_string()
}

fn default_include() -> String {
    "*.csv".to_string()
}

fn default_exclude() -> String {
    "".to_string()
}

fn default_generators() -> Vec<Generator> {
    // 返回一个默认的生成器列表，包含一个 Cocos 生成器
    vec![Generator::Cocos(CocosCodegen::default())]
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
        // 尝试从项目根目录读取 xcell.config.toml 文件
        let xcell_config_path = root.join("xcell.config.toml");
        
        if xcell_config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&xcell_config_path) {
                let mut config = Self { root: root.to_path_buf(), ..from_str(PROJECT_CONFIG).unwrap() };

                let lines: Vec<&str> = content.lines().collect();
                let mut generators = Vec::new();
                let mut current_generator = None;
                let mut project = default_project();
                let mut include = default_include();
                let mut in_line_section = false;

                for line in lines {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }

                    if line == "[line]" {
                        in_line_section = true;
                        continue;
                    }

                    if line.starts_with("[[generators]]") {
                        in_line_section = false;
                        if let Some(generator) = current_generator {
                            generators.push(generator);
                        }
                        current_generator = Some(Generator::Cocos(CocosCodegen::default()));
                        continue;
                    }

                    if in_line_section {
                        if line.starts_with("comment = ") {
                            config.line.comment = line.split('=').nth(1).unwrap().trim().parse().unwrap_or(0);
                        } else if line.starts_with("field = ") {
                            config.line.field = line.split('=').nth(1).unwrap().trim().parse().unwrap_or(0);
                        } else if line.starts_with("type = ") {
                            config.line.r#type = line.split('=').nth(1).unwrap().trim().parse().unwrap_or(0);
                        } else if line.starts_with("data = ") {
                            config.line.data = line.split('=').nth(1).unwrap().trim().parse().unwrap_or(0);
                        }
                        continue;
                    }

                    if line.starts_with("type = ") {
                        if let Some(generator) = &mut current_generator {
                            let type_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            match type_str.to_ascii_lowercase().as_str() {
                                "cocos" => {
                                    *generator = Generator::Cocos(CocosCodegen::default());
                                }
                                "typescript" => {
                                    *generator = Generator::TypeScript(TypeScriptCodegen::default());
                                }
                                _ => {}
                            }
                        }
                    } else if line.starts_with("enable = ") {
                        if let Some(generator) = &mut current_generator {
                            let enable_str = line.split('=').nth(1).unwrap().trim();
                            let enable = enable_str == "true";
                            match generator {
                                Generator::Cocos(cocos) => {
                                    cocos.enable = enable;
                                }
                                Generator::TypeScript(typescript) => {
                                    typescript.enable = enable;
                                }
                                _ => {}
                            }
                        }
                    } else if line.starts_with("project = ") {
                        if let Some(generator) = &mut current_generator {
                            let project_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            match generator {
                                Generator::Cocos(cocos) => {
                                    cocos.project = project_str.to_string();
                                }
                                Generator::TypeScript(typescript) => {
                                    typescript.project = project_str.to_string();
                                }
                                _ => {}
                            }
                        } else {
                            let project_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            project = project_str.to_string();
                        }
                    } else if line.starts_with("include = ") {
                        let include_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                        include = include_str.to_string();
                    } else if line.starts_with("loader = ") {
                        if let Some(generator) = &mut current_generator {
                            let loader_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            match generator {
                                Generator::Cocos(cocos) => {
                                    cocos.output = loader_str.to_string();
                                }
                                Generator::TypeScript(typescript) => {
                                    typescript.output = loader_str.to_string();
                                }
                                _ => {}
                            }
                        }
                    } else if line.starts_with("loader_template = ") {
                        if let Some(generator) = &mut current_generator {
                            let template_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            match generator {
                                Generator::TypeScript(typescript) => {
                                    typescript.loader_template = template_str.to_string();
                                }
                                _ => {}
                            }
                        }
                    } else if line.starts_with("storage_debug_type = ") {
                        if let Some(generator) = &mut current_generator {
                            let debug_type_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            match generator {
                                Generator::Cocos(cocos) => {
                                    match debug_type_str.to_ascii_lowercase().as_str() {
                                        "json" => {
                                            cocos.storage_debug = Some(CocosStorage::Json(CocosJsonConfig {
                                                enable: true,
                                                output: String::new(),
                                            }));
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                    } else if line.starts_with("storage_type = ") {
                        if let Some(generator) = &mut current_generator {
                            let storage_type_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            match generator {
                                Generator::TypeScript(typescript) => {
                                    typescript.storage_type = storage_type_str.to_string();
                                }
                                _ => {}
                            }
                        }
                    } else if line.starts_with("storage = ") {
                        if let Some(generator) = &mut current_generator {
                            let storage_str = line.split('=').nth(1).unwrap().trim().trim_matches('"');
                            match generator {
                                Generator::Cocos(cocos) => {
                                    if let CocosStorage::Json(json_config) = &mut cocos.storage {
                                        json_config.output = storage_str.to_string();
                                    }
                                }
                                Generator::TypeScript(typescript) => {
                                    typescript.storage = storage_str.to_string();
                                }
                                _ => {}
                            }
                        }
                    }
                }

                config.project = project;
                config.include = include;

                if let Some(generator) = current_generator {
                    generators.push(generator);
                }

                if !generators.is_empty() {
                    config.generators = generators;
                }

                return config;
            }
        } else {
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
                    if std::fs::write(&xcell_config_path, config_str).is_ok() {}
                }
            }
        }
        
        // 使用默认配置
        let basic: Self = from_str(PROJECT_CONFIG).unwrap();
        Self { root: root.to_path_buf(), ..basic }
    }
}
