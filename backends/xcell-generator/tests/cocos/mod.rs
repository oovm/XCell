use std::{collections::HashMap, fs, path::Path};
use xcell_generator::codegen::{Codegen, CodegenContext, cocos::CocosCodegen};

#[test]
fn test_cocos_code_generation() {
    // 创建临时目录
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    // 创建代码生成上下文
    let mut options = HashMap::new();
    options.insert("package_name".to_string(), "TestPackage".to_string());

    let context = CodegenContext {
        output_dir: std::path::PathBuf::from(temp_path.to_str().unwrap()),
        options,
        global_options: HashMap::new(),
    };

    // 创建 Cocos 代码生成器
    let generator = CocosCodegen::new();

    // 生成代码
    let result = generator.generate(&context);
    println!("Generate result: {:?}", result);
    result.expect("Failed to generate code");

    // 列出临时目录中的文件
    println!("Files in temp directory:");
    for entry in std::fs::read_dir(temp_path).expect("Failed to read temp directory") {
        if let Ok(entry) = entry {
            println!("  {:?}", entry.file_name());
        }
    }

    // 验证生成的文件是否存在
    let configs_json_path = temp_path.join("configs.json");
    let config_manager_ts_path = temp_path.join("ConfigManager.ts");

    assert!(configs_json_path.exists(), "configs.json file not found");
    assert!(config_manager_ts_path.exists(), "ConfigManager.ts file not found");

    // 验证生成的 TypeScript 代码包含单例模式实现
    let ts_content = fs::read_to_string(config_manager_ts_path).expect("Failed to read ConfigManager.ts");
    assert!(ts_content.contains("class ConfigManager"), "ConfigManager class not found");
    assert!(ts_content.contains("private static instance: ConfigManager"), "Singleton instance not found");
    assert!(ts_content.contains("public static getInstance()"), "getInstance method not found");
    assert!(ts_content.contains("getConfig"), "getConfig method not found");
    assert!(ts_content.contains("loadConfigs"), "loadConfigs method not found");

    println!("Cocos test completed successfully");
}
