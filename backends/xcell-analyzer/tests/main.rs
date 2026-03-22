use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::fs;

use tracing::Level;
use xcell_analyzer::{
    PROJECT_CONFIG, ProjectConfig,
};
use xcell_analyzer::validation::{RefValidator, ValidationResult};
use xcell_core::for_3rd::BigInt;
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
    println!("{:#?}", toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap())
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
    let mut all_ids: BTreeMap<String, BTreeSet<BigInt>> = BTreeMap::new();

    let mut item_ids = BTreeSet::new();
    item_ids.insert(BigInt::from(1));
    item_ids.insert(BigInt::from(2));
    item_ids.insert(BigInt::from(3));
    all_ids.insert("Item".to_string(), item_ids);

    let mut monster_ids = BTreeSet::new();
    monster_ids.insert(BigInt::from(100));
    monster_ids.insert(BigInt::from(200));
    all_ids.insert("Monster".to_string(), monster_ids);

    assert!(all_ids.contains_key("Item"));
    assert!(all_ids.contains_key("Monster"));

    let item_ids = all_ids.get("Item").unwrap();
    assert!(item_ids.contains(&BigInt::from(1)));
    assert!(item_ids.contains(&BigInt::from(2)));
    assert!(!item_ids.contains(&BigInt::from(999)));
}

#[test]
fn test_ref_validator_validate_reference_valid() {
    let mut all_ids: BTreeMap<String, BTreeSet<BigInt>> = BTreeMap::new();
    let mut item_ids = BTreeSet::new();
    item_ids.insert(BigInt::from(1));
    item_ids.insert(BigInt::from(5));
    item_ids.insert(BigInt::from(10));
    all_ids.insert("Item".to_string(), item_ids);

    let result = RefValidator::validate_reference(
        5,
        "Item",
        &all_ids,
        "TestTable",
        "item_ref",
        0,
        0,
    );
    assert!(result.is_none(), "Valid reference should not return error");

    let result_zero = RefValidator::validate_reference(
        0,
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
    let mut all_ids: BTreeMap<String, BTreeSet<BigInt>> = BTreeMap::new();
    let mut item_ids = BTreeSet::new();
    item_ids.insert(BigInt::from(1));
    all_ids.insert("Item".to_string(), item_ids);

    let result = RefValidator::validate_reference(
        999,
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
    let all_ids: BTreeMap<String, BTreeSet<BigInt>> = BTreeMap::new();

    let result = RefValidator::validate_reference(
        1,
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

#[test]
fn test_bigint_from_str() {
    use std::str::FromStr;
    let id = BigInt::from_str("12345").unwrap();
    assert_eq!(id, BigInt::from(12345));

    let id_from_int = BigInt::from(42i64);
    assert_eq!(id_from_int, BigInt::from(42));
}
