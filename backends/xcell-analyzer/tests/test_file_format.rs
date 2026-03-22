use std::{fs::File, io::Write, path::PathBuf};

use xcell_analyzer::x_table::table::{CsvTable, TableReader, TsvTable};
use xcell_analyzer::{PROJECT_CONFIG, ProjectConfig};

/// 测试 CSV 文件的正常处理
#[test]
fn test_csv_normal() {
    // 创建一个临时 CSV 文件
    let mut path = PathBuf::from("test_csv_normal.csv");
    let mut file = File::create(&path).unwrap();

    // 写入 CSV 内容
    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "int,string,int").unwrap();
    writeln!(file, "1,Alice,25").unwrap();
    writeln!(file, "2,Bob,30").unwrap();

    // 加载项目配置
    let config = toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    // 加载 CSV 文件
    let table = CsvTable::load(&path, &config).unwrap();

    // 验证表格名称
    assert_eq!(table.get_name(), "test_csv_normal");

    // 验证表头
    let header0 = table.get_header(0);
    assert_eq!(header0.field_name, "id");

    let header1 = table.get_header(1);
    assert_eq!(header1.field_name, "name");

    let header2 = table.get_header(2);
    assert_eq!(header2.field_name, "age");

    // 验证表格类型
    assert!(table.is_list());
    assert!(!table.is_dict());
    assert!(!table.is_class());

    // 清理临时文件
    std::fs::remove_file(&path).unwrap();
}

/// 测试 TSV 文件的正常处理
#[test]
fn test_tsv_normal() {
    // 创建一个临时 TSV 文件
    let mut path = PathBuf::from("test_tsv_normal.tsv");
    let mut file = File::create(&path).unwrap();

    // 写入 TSV 内容
    writeln!(file, "key\tvalue").unwrap();
    writeln!(file, "string\tstring").unwrap();
    writeln!(file, "name\tAlice").unwrap();
    writeln!(file, "age\t25").unwrap();

    // 加载项目配置
    let config = toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    // 加载 TSV 文件
    let table = TsvTable::load(&path, &config).unwrap();

    // 验证表格名称
    assert_eq!(table.get_name(), "test_tsv_normal");

    // 验证表头
    let header0 = table.get_header(0);
    assert_eq!(header0.field_name, "key");

    let header1 = table.get_header(1);
    assert_eq!(header1.field_name, "value");

    // 验证表格类型
    assert!(table.is_dict());
    assert!(!table.is_list());
    assert!(!table.is_class());

    // 清理临时文件
    std::fs::remove_file(&path).unwrap();
}

/// 测试空文件的处理
#[test]
fn test_empty_file() {
    // 创建一个临时空文件
    let mut path = PathBuf::from("test_empty.csv");
    File::create(&path).unwrap();

    // 加载项目配置
    let config = toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    // 加载空文件，应该成功但表格为空
    let table = CsvTable::load(&path, &config).unwrap();

    // 验证表格名称
    assert_eq!(table.get_name(), "test_empty");

    // 验证表头（应该返回默认值）
    let header = table.get_header(0);
    assert!(!header.complete);

    // 清理临时文件
    std::fs::remove_file(&path).unwrap();
}

/// 测试只有表头的文件
#[test]
fn test_only_header() {
    // 创建一个临时文件，只有表头
    let mut path = PathBuf::from("test_only_header.csv");
    let mut file = File::create(&path).unwrap();

    // 写入表头
    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "int,string,int").unwrap();

    // 加载项目配置
    let config = toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    // 加载文件
    let table = CsvTable::load(&path, &config).unwrap();

    // 验证表格名称
    assert_eq!(table.get_name(), "test_only_header");

    // 验证表头
    let header0 = table.get_header(0);
    assert_eq!(header0.field_name, "id");

    // 清理临时文件
    std::fs::remove_file(&path).unwrap();
}

/// 测试文件不存在的情况
#[test]
fn test_file_not_exists() {
    // 不存在的文件路径
    let path = PathBuf::from("non_existent_file.csv");

    // 加载项目配置
    let config = toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    // 尝试加载不存在的文件，应该失败
    let result = CsvTable::load(&path, &config);
    assert!(result.is_err());
}

/// 测试格式错误的文件
#[test]
fn test_invalid_format() {
    // 创建一个格式错误的文件（缺少字段）
    let mut path = PathBuf::from("test_invalid_format.csv");
    let mut file = File::create(&path).unwrap();

    // 写入内容，第二行缺少一个字段
    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "1,Alice").unwrap(); // 缺少 age 字段

    // 加载项目配置
    let config = toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    // 加载文件，应该成功（CSV 库会处理这种情况）
    let table = CsvTable::load(&path, &config).unwrap();

    // 验证表格名称
    assert_eq!(table.get_name(), "test_invalid_format");

    // 清理临时文件
    std::fs::remove_file(&path).unwrap();
}

/// 测试大文件的性能
#[test]
fn test_large_file() {
    // 创建一个大文件
    let mut path = PathBuf::from("test_large_file.csv");
    let mut file = File::create(&path).unwrap();

    // 写入表头
    writeln!(file, "id,name,age").unwrap();
    writeln!(file, "int,string,int").unwrap();

    // 写入 1000 行数据
    for i in 1..=1000 {
        writeln!(file, "{},Person{},{}", i, i, 20 + i % 50).unwrap();
    }

    // 加载项目配置
    let config = toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap();

    // 记录开始时间
    let start = std::time::Instant::now();

    // 加载大文件
    let table = CsvTable::load(&path, &config).unwrap();

    // 记录结束时间
    let duration = start.elapsed();

    // 验证表格名称
    assert_eq!(table.get_name(), "test_large_file");

    // 验证加载时间（应该在 1 秒内）
    assert!(duration.as_secs() < 1, "加载大文件时间过长: {:?}", duration);

    // 清理临时文件
    std::fs::remove_file(&path).unwrap();
}
