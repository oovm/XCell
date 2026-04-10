use std::path::PathBuf;
use xcell_provider::{CsvTable, FileFormat, FileFormatDetector, TableReader, TsvTable};

fn create_temp_csv(content: &str, name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("xcell_provider_test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    path
}

fn create_temp_tsv(content: &str, name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("xcell_provider_test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, content).unwrap();
    path
}

fn cleanup(path: &std::path::Path) {
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_csv_load_and_headers() {
    let content = "id,name,score\n1,Alice,100\n2,Bob,90\n";
    let path = create_temp_csv(content, "test_headers.csv");
    let table = CsvTable::load(&path).unwrap();

    assert_eq!(table.get_name(), "test_headers");

    let headers: Vec<_> = table.headers().collect();
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].field_name, "id");
    assert_eq!(headers[1].field_name, "name");
    assert_eq!(headers[2].field_name, "score");

    cleanup(&path);
}

#[test]
fn test_tsv_load_and_headers() {
    let content = "id\tname\tscore\n1\tAlice\t100\n2\tBob\t90\n";
    let path = create_temp_tsv(content, "test_headers.tsv");
    let table = TsvTable::load(&path).unwrap();

    assert_eq!(table.get_name(), "test_headers");

    let headers: Vec<_> = table.headers().collect();
    assert_eq!(headers.len(), 3);
    assert_eq!(headers[0].field_name, "id");
    assert_eq!(headers[1].field_name, "name");
    assert_eq!(headers[2].field_name, "score");

    cleanup(&path);
}

#[test]
fn test_ods_format_detection() {
    let dir = std::env::temp_dir().join("xcell_provider_test");
    std::fs::create_dir_all(&dir).unwrap();

    let ods_path = dir.join("test.ods");
    std::fs::write(&ods_path, "").unwrap();

    let format = FileFormatDetector::detect(&ods_path).unwrap();
    assert_eq!(format, FileFormat::Ods);

    let csv_path = dir.join("test.csv");
    std::fs::write(&csv_path, "").unwrap();
    let format = FileFormatDetector::detect(&csv_path).unwrap();
    assert_eq!(format, FileFormat::Csv);

    let tsv_path = dir.join("test.tsv");
    std::fs::write(&tsv_path, "").unwrap();
    let format = FileFormatDetector::detect(&tsv_path).unwrap();
    assert_eq!(format, FileFormat::Tsv);

    let xlsx_path = dir.join("test.xlsx");
    std::fs::write(&xlsx_path, "").unwrap();
    let format = FileFormatDetector::detect(&xlsx_path).unwrap();
    assert_eq!(format, FileFormat::Excel);

    cleanup(&ods_path);
    cleanup(&csv_path);
    cleanup(&tsv_path);
    cleanup(&xlsx_path);
}

#[test]
fn test_csv_parse_type() {
    let content = "id,name,score\n1,Alice,100\n";
    let path = create_temp_csv(content, "test_parse_type.csv");
    let table = CsvTable::load(&path).unwrap();

    let bool_type = table.parse_type("bool");
    assert!(matches!(bool_type, xcell_core::XCellTyped::Boolean(_)));

    let int_type = table.parse_type("int");
    assert!(matches!(int_type, xcell_core::XCellTyped::Integer(_)));

    let string_type = table.parse_type("string");
    assert!(matches!(string_type, xcell_core::XCellTyped::String(_)));

    cleanup(&path);
}

#[test]
fn test_csv_rows_streaming() {
    let content = "id,name\n1,Alice\n2,Bob\n3,Charlie\n";
    let path = create_temp_csv(content, "test_streaming.csv");
    let table = CsvTable::load(&path).unwrap();

    let mut rows_iter = table.rows();

    let (idx0, row0) = rows_iter.next().unwrap();
    assert_eq!(idx0, 0);
    assert_eq!(row0.len(), 2);

    let (idx1, row1) = rows_iter.next().unwrap();
    assert_eq!(idx1, 1);
    assert_eq!(row1.len(), 2);

    let (idx2, row2) = rows_iter.next().unwrap();
    assert_eq!(idx2, 2);
    assert_eq!(row2.len(), 2);

    assert!(rows_iter.next().is_none());

    cleanup(&path);
}
