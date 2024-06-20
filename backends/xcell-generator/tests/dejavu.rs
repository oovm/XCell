use std::path::Path;
use tempfile::tempdir;

use xcell_generator::{
    Generator,
    config::{GeneratorConfig, ProductConfig, ProductType},
};

#[test]
fn test_dejavu_codegen() {
    // 初始化日志系统
    Generator::init_logging();

    // 创建临时目录作为输出目录
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().to_str().unwrap();

    // 创建配置
    let mut config = GeneratorConfig::default();
    let product = ProductConfig {
        product_type: ProductType::Dejavu,
        output_dir: output_dir.to_string(),
        options: Default::default(),
        enabled: true,
    };
    config.products.push(product);

    // 创建生成器实例
    let generator = Generator::new(config);

    // 执行生成
    let result = generator.generate();

    // 验证生成成功
    assert!(result.is_ok());

    // 验证输出目录存在
    assert!(Path::new(output_dir).exists());

    // 验证生成的文件存在
    let hello_rs_path = Path::new(output_dir).join("hello.rs");
    assert!(hello_rs_path.exists());

    // 读取生成的文件内容
    let content = std::fs::read_to_string(&hello_rs_path).unwrap();

    // 验证生成的代码包含必要的导入和宏使用
    assert!(content.contains("use dejavu_macros::Template"));
    assert!(content.contains("use dejavu_types::values::Context"));
    assert!(content.contains("#[derive(Template)]"));
    assert!(content.contains("#[template(path = \"templates/hello.rs.dejavu\")]"));
    assert!(content.contains("pub struct helloTemplate"));
}
