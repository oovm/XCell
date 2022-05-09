use std::path::PathBuf;

use xcell_core::{utils::first_is_nil, ProjectConfig, XCellTable, XResult};

#[test]
fn test2() -> XResult {
    let path = "tests/test_buffer/BuffTable.xlsx";
    let config = ProjectConfig::default();
    let xc = XCellTable::load_file(PathBuf::from(path), &config)?;
    println!("{:#?}", xc);
    for row in xc.table.rows().skip(3) {
        if first_is_nil(row) {
            continue;
        }
        println!("{:?}", row);
    }
    Ok(())
}
