use std::path::PathBuf;

use calamine::{open_workbook_auto, DataType, Reader};

use xcell_core::{XCellTable, XResult};

#[test]
fn test2() -> XResult {
    let path = "tests/test_buffer/BuffTable.xlsx";
    let xc = XCellTable::load_file(&PathBuf::from(path))?;
    println!("{:#?}", xc);
    Ok(())
}
