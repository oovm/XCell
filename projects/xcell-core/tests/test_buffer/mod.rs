#![allow(unused)]

use xcell_core::{
    codegen::UnityCodegen,
    utils::{build_glob_set, walk_blob_set},
    Failure, ProjectConfig, Success, XCellTable, XResult,
};

use super::*;

#[test]
fn test2() -> XResult {
    logger();
    log::debug!("this is a debug {}", "message");
    log::error!("this is printed by default");
    let root = PathBuf::from("./");
    let config = ProjectConfig::default();
    let unity = UnityCodegen::default();
    let set = build_glob_set("tests/*.xlsx")?;

    for excel in walk_blob_set(&root, &set).unwrap() {
        match XCellTable::load_file(&excel, &config) {
            Success { value, diagnostics } => unity.write_class(&value)?,
            Failure { fatal, diagnostics } => {}
        }
    }
    Ok(())
}
