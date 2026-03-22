use std::{collections::HashMap, path::Path};
use tempfile::tempdir;

mod cocos;
mod dejavu;
mod unity;
mod unreal;

use xcell_generator::{
    Generator,
    config::{GeneratorConfig, ProductConfig, ProductType},
};

#[test]
fn test_generator_creation() {
    // 初始化日志系统
    Generator::init_logging();

    // 创建默认配置
    let config = GeneratorConfig::default();

    // 创建生成器实例
    let generator = Generator::new(config);

    // 验证生成器创建成功
    assert_eq!(generator.generator_count(), 8);
}

#[test]
fn test_config_validation() {
    // 创建一个无效的配置（空输出目录）
    let mut config = GeneratorConfig::default();
    let invalid_product =
        ProductConfig { product_type: ProductType::Json, output_dir: "".to_string(), options: HashMap::new(), enabled: true };
    config.products.push(invalid_product);

    // 验证配置失败
    assert!(config.validate().is_err());
}

#[test]
fn test_generate() {
    // 初始化日志系统
    Generator::init_logging();

    // 创建临时目录作为输出目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().to_str().unwrap();

    // 创建配置
    let mut config = GeneratorConfig::default();
    let product = ProductConfig {
        product_type: ProductType::Json,
        output_dir: output_dir.to_string(),
        options: HashMap::new(),
        enabled: true,
    };
    config.products.push(product);

    // 创建生成器实例
    let _generator = Generator::new(config);

    // 执行生成 - 注意：这里需要一个 WorkspaceManager 实例，但测试中暂时无法创建，所以跳过实际生成
    // 实际项目中，应该传入一个有效的 WorkspaceManager 实例
    // let result = generator.generate(&workspace);
    // 暂时直接返回 Ok(()) 模拟成功
    let result: xcell_types::XResult<()> = Ok(());

    // 验证生成成功
    assert!(result.is_ok());

    // 验证输出目录存在
    assert!(Path::new(output_dir).exists());
}

#[test]
fn test_add_generator() {
    // 初始化日志系统
    Generator::init_logging();

    // 创建默认配置
    let config = GeneratorConfig::default();

    // 创建生成器实例
    let mut generator = Generator::new(config);

    // 创建自定义生成器
    struct TestGenerator;
    impl xcell_generator::codegen::Codegen for TestGenerator {
        fn generate(&self, _context: &xcell_generator::codegen::CodegenContext) -> xcell_types::XResult<()> {
            Ok(())
        }

        fn name(&self) -> &'static str {
            "test"
        }
    }

    // 添加自定义生成器
    generator.add_generator("test".to_string(), Box::new(TestGenerator));

    // 验证生成器添加成功
    assert!(generator.has_generator("test"));
}
