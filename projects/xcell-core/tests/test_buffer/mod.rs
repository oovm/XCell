use std::path::PathBuf;

use xcell_core::{codegen::UnityCodegen, utils::build_glob, ProjectConfig, XCellTable, XResult};

#[test]
fn test2() -> XResult {
    let root = PathBuf::from("/").canonicalize().unwrap();
    let glob = build_glob("tests/test_buffer/*.xlsx").unwrap();

    let config = ProjectConfig::default();
    let xc = XCellTable::load_file(&PathBuf::from(path), &config).unwrap();
    let unity = UnityCodegen::default();
    // code.write_csharp(&xc, &PathBuf::from("tests/test_buffer/BufferTable.cs")).unwrap();
    unity.write_class(&xc).unwrap();
    Ok(())
}
