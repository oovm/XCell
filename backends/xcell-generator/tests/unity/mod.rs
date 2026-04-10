use std::{collections::HashMap, fs, path::Path};
use xcell_generator::codegen::{Codegen, CodegenContext, unity::UnityGenerator};
use xcell_config::UnityCodegen;

#[test]
fn test_unity_generator_creation() {
    let generator = UnityGenerator::default_generator();
    assert_eq!(generator.name(), "unity");
}

#[test]
fn test_unity_config_validate() {
    let mut config = UnityCodegen::default();
    assert!(config.validate().is_err());

    config.output = "Assets/Scripts/Generated".to_string();
    config.namespace = "DataTable".to_string();
    config.manager = "DataTableManager".to_string();
    assert!(config.validate().is_ok());
}

#[test]
fn test_unity_config_validate_invalid_namespace() {
    let mut config = UnityCodegen::default();
    config.output = "Assets/Scripts/Generated".to_string();
    config.namespace = "Invalid Namespace!".to_string();
    config.manager = "DataTableManager".to_string();
    assert!(config.validate().is_err());
}

#[test]
fn test_unity_generate_without_workspace() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    let options = HashMap::new();
    let context = CodegenContext {
        output_dir: std::path::PathBuf::from(temp_path.to_str().unwrap()),
        options,
        global_options: HashMap::new(),
        workspace: None,
    };

    let mut config = UnityCodegen::default();
    config.output = "Assets/Scripts/Generated".to_string();
    config.namespace = "DataTable".to_string();
    config.manager = "DataTableManager".to_string();

    let generator = UnityGenerator::new(config);
    let result = generator.generate(&context);
    assert!(result.is_err());
}
