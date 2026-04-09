use std::{fs::File, io::Write, path::PathBuf};

use xcell_provider::{CsvTable, TsvTable, TableReader};
use xcell_analyzer::{PROJECT_CONFIG, ProjectConfig};
use oak_toml::from_str;

/// 测试 CSV 文件的正常处理
#[test]
fn test_csv_normal() {
    let mut path = PathBuf::from("test_csv_normal.csv");
    let mut file = File::create(&path).unwrap();

    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "int,string,int").unwrap();
    writeln!(file, "1,Alice,25").unwrap();
    writeln!(file, "2,Bob,30").unwrap();

    let config = from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    let table = CsvTable::load(&path).unwrap();

    assert_eq!(table.get_name(), "test_csv_normal");

    let header0 = table.get_header(0);
    assert_eq!(header0.field_name, "id");

    let header1 = table.get_header(1);
    assert_eq!(header1.field_name, "name");

    let header2 = table.get_header(2);
    assert_eq!(header2.field_name, "age");

    std::fs::remove_file(&path).unwrap();
}

/// 测试 TSV 文件的正常处理
#[test]
fn test_tsv_normal() {
    let mut path = PathBuf::from("test_tsv_normal.tsv");
    let mut file = File::create(&path).unwrap();

    writeln!(file, "key\tvalue").unwrap();
    writeln!(file, "string\tstring").unwrap();
    writeln!(file, "name\tAlice").unwrap();
    writeln!(file, "age\t25").unwrap();

    let config = from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    let table = TsvTable::load(&path).unwrap();

    assert_eq!(table.get_name(), "test_tsv_normal");

    let header0 = table.get_header(0);
    assert_eq!(header0.field_name, "key");

    let header1 = table.get_header(1);
    assert_eq!(header1.field_name, "value");

    std::fs::remove_file(&path).unwrap();
}

/// 测试空文件的处理
#[test]
fn test_empty_file() {
    let mut path = PathBuf::from("test_empty.csv");
    File::create(&path).unwrap();

    let config = from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    let table = CsvTable::load(&path).unwrap();

    assert_eq!(table.get_name(), "test_empty");

    let header = table.get_header(0);
    assert!(!header.complete);

    std::fs::remove_file(&path).unwrap();
}

/// 测试只有表头的文件
#[test]
fn test_only_header() {
    let mut path = PathBuf::from("test_only_header.csv");
    let mut file = File::create(&path).unwrap();

    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "int,string,int").unwrap();

    let config = from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    let table = CsvTable::load(&path).unwrap();

    assert_eq!(table.get_name(), "test_only_header");

    let header0 = table.get_header(0);
    assert_eq!(header0.field_name, "id");

    std::fs::remove_file(&path).unwrap();
}

/// 测试文件不存在的情况
#[test]
fn test_file_not_exists() {
    let path = PathBuf::from("non_existent_file.csv");

    let config = from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    let result = CsvTable::load(&path);
    assert!(result.is_err());
}

/// 测试格式错误的文件
#[test]
fn test_invalid_format() {
    let mut path = PathBuf::from("test_invalid_format.csv");
    let mut file = File::create(&path).unwrap();

    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "1,Alice").unwrap();

    let config = from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    let table = CsvTable::load(&path).unwrap();

    assert_eq!(table.get_name(), "test_invalid_format");

    std::fs::remove_file(&path).unwrap();
}

/// 测试大文件的性能
#[test]
fn test_large_file() {
    let mut path = PathBuf::from("test_large_file.csv");
    let mut file = File::create(&path).unwrap();

    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "int,string,int").unwrap();

    for i in 1..=1000 {
        writeln!(file, "{},Person{},{}", i, i, 20 + i % 50).unwrap();
    }

    let config = from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    let start = std::time::Instant::now();

    let table = CsvTable::load(&path).unwrap();

    let duration = start.elapsed();

    assert_eq!(table.get_name(), "test_large_file");

    assert!(duration.as_secs() < 1, "加载大文件时间过长: {:?}", duration);

    std::fs::remove_file(&path).unwrap();
}
