use std::path::PathBuf;
use xcell_core::{utils::first_is_nil, XCellMetaInfo, XCellTable, XResult};

#[test]
fn test2() -> XResult {
    let path = "tests/test_buffer/BuffTable.xlsx";
    let xc = XCellTable::load_file(&PathBuf::from(path))?;
    for row in xc.table.rows().skip(3) {
        if first_is_nil(row) {
            continue;
        }
        println!("{:?}", row);
    }
    Ok(())
}

#[test]
fn dump_toml() -> XResult {
    let path = "tests/test_buffer/BuffTable.toml";
    let cfg = XCellMetaInfo::load_file(&PathBuf::from(path))?;
    println!("{:#?}", cfg);
    Ok(())
}
