#![warn(missing_docs)]

use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{error, info};
use xcell_analyzer::{WorkspaceManager, XClassData, XDictData, XEnumerateData, XListData};
use xcell_core::XResult;

/// 错误处理模块
pub mod error;
use error::GeneratorErrorExt;

/// 日志模块
pub mod logging;
use logging::init_logging;

/// XCell Generator
///
/// 负责生成各种格式的代码和数据文件
pub mod codegen;
use codegen::CodegenContext;

/// 配置模块
pub mod config;
use config::GeneratorConfig;

/// 模板模块
pub mod template;

// 从 xcell-analyzer 重新导出的类型
// pub use xcell_analyzer::{WorkspaceManager, XClassData, XDictData, XEnumerateData, XListData};

/// 生成器主入口
pub struct Generator {
    /// 生成器配置
    config: GeneratorConfig,
    /// 代码生成器实例映射
    generators: HashMap<String, Box<dyn codegen::Codegen>>,
}

impl Generator {
    /// 初始化日志系统
    pub fn init_logging() {
        let config = logging::LogConfig::default();
        init_logging(config);
        info!("日志系统初始化完成");
    }

    /// 创建新的生成器实例
    pub fn new(config: GeneratorConfig) -> Self {
        let mut generators: HashMap<String, Box<dyn codegen::Codegen>> = HashMap::new();

        // 初始化各种代码生成器
        generators.insert("json".to_string(), Box::new(codegen::json::JsonCodegen::default()) as Box<dyn codegen::Codegen>);
        generators.insert("binary".to_string(), Box::new(codegen::binary::BinaryCodegen::default()) as Box<dyn codegen::Codegen>);
        generators.insert("typescript".to_string(), Box::new(codegen::typescript::TypeScriptCodegen::default()) as Box<dyn codegen::Codegen>);
        generators.insert("unity".to_string(), Box::new(codegen::unity::UnityGenerator::default_generator()) as Box<dyn codegen::Codegen>);

        info!("生成器实例创建完成，注册了 {} 个代码生成器", generators.len());

        Generator { config, generators }
    }

    /// 生成代码和数据
    pub fn generate(&self, workspace: &WorkspaceManager) -> XResult<()> {
        // 验证配置
        self.config.validate()?;

        let enabled_products = self.config.enabled_products();
        info!("开始生成代码和数据，共 {} 个启用的产物", enabled_products.len());

        // 遍历所有启用的产物
        for product in enabled_products {
            let product_type_str: String = product.product_type.clone().into();
            info!("开始处理产物类型: {}", product_type_str);

            if let Some(generator) = self.generators.get(&product_type_str) {
                // 创建代码生成上下文
                // 输出目录相对于工作空间根目录
                let output_dir = workspace.config.root.join(&product.output_dir);
                
                let context = CodegenContext::new_with_workspace(
                    output_dir.to_str().unwrap_or(&product.output_dir),
                    product.options.clone(),
                    self.config.global.options.clone(),
                    workspace
                );

                // 确保输出目录存在
                if let Err(e) = context.ensure_output_dir() {
                    error!("创建输出目录失败: {:?}", e);
                    return Err(e);
                }

                // 生成对应格式的代码
                match generator.generate(&context) {
                    Ok(_) => {
                        info!("成功生成产物: {}", product_type_str);
                    }
                    Err(e) => {
                        error!("生成产物 {} 失败: {:?}", product_type_str, e);
                        return Err(e);
                    }
                }
            }
            else {
                let error = GeneratorErrorExt::generator_not_found(&product_type_str);
                error!("{}", error);
                return Err(error);
            }
        }

        info!("所有产物生成完成");
        Ok(())
    }

    /// 添加自定义代码生成器
    pub fn add_generator(&mut self, name: String, generator: Box<dyn codegen::Codegen>) {
        self.generators.insert(name, generator);
    }

    /// 组合多个生成器的输出
    pub fn generate_combined(&self) -> XResult<()> {
        // 实现组合生成逻辑
        // 例如：先生成JSON数据，然后生成TypeScript reader来读取这些JSON数据
        Ok(())
    }

    /// 获取生成器数量
    pub fn generator_count(&self) -> usize {
        self.generators.len()
    }

    /// 检查是否包含指定名称的生成器
    pub fn has_generator(&self, name: &str) -> bool {
        self.generators.contains_key(name)
    }
}
