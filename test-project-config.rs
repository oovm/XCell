use std::path::Path;
use xcell_config::ProjectConfig;

fn main() {
    // 测试目录路径
    let test_dir = Path::new("examples/test-rpg");
    
    println!("Testing ProjectConfig::new()...");
    println!("Test directory: {}", test_dir.display());
    
    // 调用 ProjectConfig::new()
    let config = ProjectConfig::new(test_dir);
    
    println!("ProjectConfig created successfully");
    println!("Root: {}", config.root.display());
    println!("Version: {}", config.version);
    println!("Include: {}", config.include);
    
    // 检查是否创建了 ProjectSettings.toml 文件
    let settings_path = test_dir.join("ProjectSettings.toml");
    if settings_path.exists() {
        println!("✓ ProjectSettings.toml created successfully!");
        // 检查文件内容
        if let Ok(content) = std::fs::read_to_string(&settings_path) {
            if content.len() > 0 {
                println!("✓ ProjectSettings.toml has content");
                println!("Content preview:");
                println!("{}", content);
            } else {
                println!("⚠ ProjectSettings.toml is empty");
            }
        }
    } else {
        println!("✗ ProjectSettings.toml was not created");
    }
}
