use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use xcell_config::project::Generator;

use crate::error::{GeneratorError, GeneratorErrorExt, GeneratorResult};

/// 产物类型
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum ProductType {
    /// Unity 代码
    Unity,
    /// Cocos 代码
    Cocos,
    /// XLua 代码
    Xlua,
    /// SQL 代码
    Sql,
    /// JSON 数据
    Json,
    /// TypeScript 代码
    TypeScript,
    /// Dejavu 模板
    Dejavu,
    /// 自定义类型
    Custom(String),
}

impl From<ProductType> for String {
    fn from(product_type: ProductType) -> Self {
        match product_type {
            ProductType::Unity => "unity".to_string(),
            ProductType::Cocos => "cocos".to_string(),
            ProductType::Xlua => "xlua".to_string(),
            ProductType::Sql => "sql".to_string(),
            ProductType::Json => "json".to_string(),
            ProductType::TypeScript => "typescript".to_string(),
            ProductType::Dejavu => "dejavu".to_string(),
            ProductType::Custom(s) => s,
        }
    }
}

impl From<String> for ProductType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "unity" => ProductType::Unity,
            "cocos" => ProductType::Cocos,
            "xlua" => ProductType::Xlua,
            "sql" => ProductType::Sql,
            "json" => ProductType::Json,
            "typescript" => ProductType::TypeScript,
            "dejavu" => ProductType::Dejavu,
            _ => ProductType::Custom(s),
        }
    }
}

/// 产物配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProductConfig {
    /// 产物类型
    pub product_type: ProductType,
    /// 输出目录
    pub output_dir: String,
    /// 其他配置参数
    pub options: HashMap<String, String>,
    /// 是否启用
    pub enabled: bool,
}

impl Default for ProductConfig {
    fn default() -> Self {
        Self { product_type: ProductType::Json, output_dir: "./output".to_string(), options: HashMap::new(), enabled: true }
    }
}

/// 全局配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GlobalConfig {
    /// 并行处理线程数
    pub parallel_threads: Option<usize>,
    /// 日志级别
    pub log_level: Option<String>,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 其他全局配置参数
    pub options: HashMap<String, String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self { parallel_threads: None, log_level: Some("info".to_string()), enable_cache: true, options: HashMap::new() }
    }
}

/// 生成器配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeneratorConfig {
    /// 全局配置
    pub global: GlobalConfig,
    /// 启用的产物列表
    pub products: Vec<ProductConfig>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self { global: GlobalConfig::default(), products: Vec::new() }
    }
}

impl GeneratorConfig {
    /// 从文件加载配置
    pub fn from_file(path: &Path) -> GeneratorResult<Self> {
        let mut file = File::open(path).map_err(|e| -> GeneratorError {
            crate::error::GeneratorErrorExt::config_error(&format!("无法打开配置文件: {:?}", e))
        })?;

        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| -> GeneratorError {
            crate::error::GeneratorErrorExt::config_error(&format!("无法读取配置文件: {:?}", e))
        })?;

        oak_toml::from_str(&content).map_err(|e| -> GeneratorError {
            crate::error::GeneratorErrorExt::config_error(&format!("配置文件格式错误: {:?}", e))
        })
    }

    /// 验证配置
    pub fn validate(&self) -> GeneratorResult<()> {
        // 验证产物配置
        for (index, product) in self.products.iter().enumerate() {
            if product.output_dir.is_empty() {
                return Err(GeneratorErrorExt::config_error(&format!("产物 {} 的输出目录不能为空", index)));
            }
        }

        Ok(())
    }

    /// 获取启用的产物
    pub fn enabled_products(&self) -> Vec<&ProductConfig> {
        self.products.iter().filter(|p| p.enabled).collect()
    }

    /// 从 ProjectConfig 创建 GeneratorConfig
    pub fn from_project_config(project_config: &xcell_config::ProjectConfig) -> Self {
        let mut products = Vec::new();

        // 处理生成器列表
        for generator in &project_config.generators {
            match generator {
                xcell_config::project::Generator::Unity(unity) => {
                    if unity.enable {
                        let mut options = std::collections::HashMap::new();
                        options.insert("namespace".to_string(), unity.namespace.clone());
                        options.insert("output".to_string(), unity.output.clone());
                        
                        products.push(ProductConfig {
                            product_type: ProductType::Unity,
                            output_dir: unity.output.clone(),
                            options,
                            enabled: true,
                        });
                    }
                }
                xcell_config::project::Generator::Cocos(cocos) => {
                    if cocos.enable {
                        let mut options = std::collections::HashMap::new();
                        options.insert("output".to_string(), cocos.output.clone());
                        
                        products.push(ProductConfig {
                            product_type: ProductType::Cocos,
                            output_dir: cocos.output.clone(),
                            options,
                            enabled: true,
                        });
                    }
                }
                _ => {
                    // 处理其他类型的生成器
                }
            }
        }



        GeneratorConfig {
            global: GlobalConfig::default(),
            products,
        }
    }
}
