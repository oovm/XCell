use std::path::PathBuf;

use xcell_core::{codegen::UnityCodegen, ProjectConfig, XCellTable, XResult};

#[test]
fn test2() -> XResult {
    let path = "tests/test_buffer/Buff.xlsx";
    let config = ProjectConfig::default();
    let xc = XCellTable::load_file(PathBuf::from(path), &config).unwrap();
    let code = UnityCodegen::default();

    code.write_csharp(&xc, &PathBuf::from("tests/test_buffer/BufferTable.cs")).unwrap();
    Ok(())
}
