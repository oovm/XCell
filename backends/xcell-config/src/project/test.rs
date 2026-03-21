use std::path::Path;
use super::ProjectConfig;

#[test]
fn test_old_format_config() {
    // 测试旧格式配置文件解析
    let old_config_path = Path::new("e:\\灵之镜有限公司\\XCell\\examples\\rpg-untyped\\ProjectSettings.toml");
    let config = ProjectConfig::new(old_config_path);
    assert!(!config.generators.is_empty(), "Old format config should have generators!");
    println!("Old format config parsed successfully!");
    println!("Generators count: {}", config.generators.len());
    for (i, generator) in config.generators.iter().enumerate() {
        println!("Generator {} type: {:?}", i, generator.r#type);
    }
}

#[test]
fn test_new_format_config() {
    // 测试新格式配置文件解析
    let new_config_path = Path::new("e:\\灵之镜有限公司\\XCell\\backends\\xcell-config\\ProjectConfig.toml");
    let config = ProjectConfig::new(new_config_path);
    assert!(!config.generators.is_empty(), "New format config should have generators!");
    println!("New format config parsed successfully!");
    println!("Generators count: {}", config.generators.len());
    for (i, generator) in config.generators.iter().enumerate() {
        println!("Generator {} type: {:?}", i, generator.r#type);
    }
}
