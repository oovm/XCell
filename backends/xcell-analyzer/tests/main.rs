use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::fs;

use tracing::Level;
use xcell_analyzer::{
    PROJECT_CONFIG, ProjectConfig,
};
use xcell_analyzer::validation::{RefValidator, ValidationResult};
use xcell_core::XError;

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
    println!("{:#?}", oak_toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap())
}

#[test]
fn test_project_settings_creation() {
    let test_dir = PathBuf::from("test-project-settings");
    if !test_dir.exists() {
        fs::create_dir(&test_dir).unwrap();
    }

    println!("Testing ProjectSettings.toml creation in: {}", test_dir.display());

    let config = ProjectConfig::new(&test_dir);

    println!("ProjectConfig created successfully");
    println!("Root: {}", config.root.display());

    let settings_path = test_dir.join("ProjectSettings.toml");
    assert!(settings_path.exists(), "ProjectSettings.toml should be created");

    let content = fs::read_to_string(&settings_path).unwrap();
    assert!(!content.is_empty(), "ProjectSettings.toml should not be empty");

    println!("✓ ProjectSettings.toml created successfully with content");
    println!("Content: {}", content);

    fs::remove_dir_all(&test_dir).unwrap();
}

#[test]
fn test_ref_validator_collect_all_ids() {
    let mut all_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    let mut item_ids = BTreeSet::new();
    item_ids.insert("1".to_string());
    item_ids.insert("2".to_string());
    item_ids.insert("3".to_string());
    all_ids.insert("Item".to_string(), item_ids);

    let mut monster_ids = BTreeSet::new();
    monster_ids.insert("100".to_string());
    monster_ids.insert("200".to_string());
    all_ids.insert("Monster".to_string(), monster_ids);

    assert!(all_ids.contains_key("Item"));
    assert!(all_ids.contains_key("Monster"));

    let item_ids = all_ids.get("Item").unwrap();
    assert!(item_ids.contains("1"));
    assert!(item_ids.contains("2"));
    assert!(!item_ids.contains("999"));
}

#[test]
fn test_ref_validator_validate_reference_valid() {
    let mut all_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut item_ids = BTreeSet::new();
    item_ids.insert("1".to_string());
    item_ids.insert("5".to_string());
    item_ids.insert("10".to_string());
    all_ids.insert("Item".to_string(), item_ids);

    let result = RefValidator::validate_reference(
        "5",
        "Item",
        &all_ids,
        "TestTable",
        "item_ref",
        0,
        0,
    );
    assert!(result.is_none(), "Valid reference should not return error");

    let result_zero = RefValidator::validate_reference(
        "0",
        "Item",
        &all_ids,
        "TestTable",
        "item_ref",
        0,
        0,
    );
    assert!(result_zero.is_none(), "Zero reference (null) should not return error");
}

#[test]
fn test_ref_validator_validate_reference_invalid_id() {
    let mut all_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut item_ids = BTreeSet::new();
    item_ids.insert("1".to_string());
    all_ids.insert("Item".to_string(), item_ids);

    let result = RefValidator::validate_reference(
        "999",
        "Item",
        &all_ids,
        "TestTable",
        "item_ref",
        0,
        0,
    );
    assert!(result.is_some(), "Invalid reference ID should return error");
    let error = result.unwrap();
    assert!(error.to_string().contains("999"));
    assert!(error.to_string().contains("Item"));
}

#[test]
fn test_ref_validator_validate_reference_invalid_table() {
    let all_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    let result = RefValidator::validate_reference(
        "1",
        "NonExistentTable",
        &all_ids,
        "TestTable",
        "ref_field",
        0,
        0,
    );
    assert!(result.is_some(), "Reference to non-existent table should return error");
    let error = result.unwrap();
    assert!(error.to_string().contains("NonExistentTable"));
}

#[test]
fn test_ref_validator_validate_string_reference() {
    let mut all_ids: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut language_ids = BTreeSet::new();
    language_ids.insert("ui/start".to_string());
    language_ids.insert("ui/settings".to_string());
    language_ids.insert("ui/exit".to_string());
    all_ids.insert("Language".to_string(), language_ids);

    let result = RefValidator::validate_reference(
        "ui/start",
        "Language",
        &all_ids,
        "TestTable",
        "text_ref",
        0,
        0,
    );
    assert!(result.is_none(), "Valid string reference should not return error");

    let result_invalid = RefValidator::validate_reference(
        "ui/invalid",
        "Language",
        &all_ids,
        "TestTable",
        "text_ref",
        0,
        0,
    );
    assert!(result_invalid.is_some(), "Invalid string reference should return error");
}

#[test]
fn test_validation_result_new() {
    let result = ValidationResult::new();
    assert!(!result.has_errors());
    assert!(result.errors.is_empty());
}

#[test]
fn test_validation_result_add_error() {
    let mut result = ValidationResult::new();
    assert!(!result.has_errors());

    result.add_error(XError::runtime_error("Test error"));
    assert!(result.has_errors());
    assert_eq!(result.errors.len(), 1);
}

#[test]
fn test_validation_result_merge() {
    let mut result1 = ValidationResult::new();
    result1.add_error(XError::runtime_error("Error 1"));

    let mut result2 = ValidationResult::new();
    result2.add_error(XError::runtime_error("Error 2"));
    result2.add_error(XError::runtime_error("Error 3"));

    result1.merge(result2);
    assert_eq!(result1.errors.len(), 3);
}
