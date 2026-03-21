use std::path::Path;
use xcell_config::project::ProjectConfig;

fn main() {
    // 测试旧格式配置文件解析
    println!("Testing old format config...");
    let old_config_path = Path::new("e:\\灵之镜有限公司\\XCell\\examples\\rpg-untyped\\ProjectSettings.toml");
    if let Ok(config) = ProjectConfig::new(old_config_path) {
        println!("Old format config parsed successfully!");
        println!("Generators count: {}", config.generators.len());
        for (i, generator) in config.generators.iter().enumerate() {
            println!("Generator {} type: {:?}", i, generator.r#type);
        }
    } else {
        println!("Failed to parse old format config!");
    }
    
    println!("\nTesting new format config...");
    let new_config_path = Path::new("e:\\灵之镜有限公司\\XCell\\backends\\xcell-config\\ProjectConfig.toml");
    if let Ok(config) = ProjectConfig::new(new_config_path) {
        println!("New format config parsed successfully!");
        println!("Generators count: {}", config.generators.len());
        for (i, generator) in config.generators.iter().enumerate() {
            println!("Generator {} type: {:?}", i, generator.r#type);
        }
    } else {
        println!("Failed to parse new format config!");
    }
}
