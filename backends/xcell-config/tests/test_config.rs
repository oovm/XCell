use std::path::Path;
use xcell_config::project::ProjectConfig;

fn main() {
    // 测试旧格式配置文件解析
    println!("Testing old format config...");
    let old_config_path = Path::new("../../../examples/rpg/ProjectSettings.toml");
    let config = ProjectConfig::new(old_config_path);
    println!("Old format config parsed successfully!");
    println!("Generators count: {}", config.generators.len());
    for (i, generator) in config.generators.iter().enumerate() {
        println!("Generator {} type: {:?}", i, generator.r#type);
    }
    
    println!("\nTesting new format config...");
    let new_config_path = Path::new("./ProjectConfig.toml");
    let config = ProjectConfig::new(new_config_path);
    println!("New format config parsed successfully!");
    println!("Generators count: {}", config.generators.len());
    for (i, generator) in config.generators.iter().enumerate() {
        println!("Generator {} type: {:?}", i, generator.r#type);
    }
}
