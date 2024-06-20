use tracing::Level;
use xcell_core::{PROJECT_CONFIG, ProjectConfig};
use std::path::{Path, PathBuf};
use std::fs;

// use sled_typed::{Database, DiskMap};

mod test_buffer;

#[test]
fn ready() {
    println!("it works!")
}

pub fn logger() {
    let _ = tracing_subscriber::fmt().with_max_level(Level::TRACE).try_init().unwrap();
}

#[test]
fn project_config_default() {
    println!("{:#?}", toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap())
}

#[test]
fn test_project_settings_creation() {
    // 创建临时测试目录
    let test_dir = PathBuf::from("test-project-settings");
    if !test_dir.exists() {
        fs::create_dir(&test_dir).unwrap();
    }
    
    println!("Testing ProjectSettings.toml creation in: {}", test_dir.display());
    
    // 调用 ProjectConfig::new()
    let config = ProjectConfig::new(&test_dir);
    
    println!("ProjectConfig created successfully");
    println!("Root: {}", config.root.display());
    
    // 检查是否创建了 ProjectSettings.toml 文件
    let settings_path = test_dir.join("ProjectSettings.toml");
    assert!(settings_path.exists(), "ProjectSettings.toml should be created");
    
    // 检查文件内容
    let content = fs::read_to_string(&settings_path).unwrap();
    assert!(!content.is_empty(), "ProjectSettings.toml should not be empty");
    
    println!("✓ ProjectSettings.toml created successfully with content");
    println!("Content: {}", content);
    
    // 清理临时目录
    fs::remove_dir_all(&test_dir).unwrap();
}

// #[test]
// fn test_files() {
//     let path = PathBuf::from("sqlite");
//     let db = Database::open(&path).unwrap();
//     println!("{:#?}", db);
//     let map: DiskMap<String, String> = db.document("").unwrap();
//     let key = "key".to_string();
//     let value = "value".to_string();
//     map.insert(key, value);
//     println!("{:?}", map.get("key".to_string()));
//     // file_db.test().await.unwrap()
// }
