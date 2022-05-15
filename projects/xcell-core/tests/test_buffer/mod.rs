use std::path::PathBuf;

use xcell_core::{ProjectConfig, XCellTable, XResult};

#[test]
fn test2() -> XResult {
    let path = "tests/test_buffer/BuffTable.xlsx";
    let config = ProjectConfig::default();
    let xc = XCellTable::load_file(PathBuf::from(path), &config).unwrap();
    println!("{:#?}", xc.data);
    Ok(())
}
