#![allow(unused)]

use xcell_core::{
    codegen::UnityCodegen,
    utils::{build_glob_set, walk_blob_set},
    Failure, WorkspaceManager, Success, XCellTable, XResult,
};

use super::*;

#[tokio::test]
async fn test2() -> XResult {
    logger();
    let root = PathBuf::from("./");
    let config = WorkspaceManager::default();

    let unity = UnityCodegen::default();
    let set = build_glob_set("tests/*.xlsx")?;

    for excel in walk_blob_set(&root, &set).await.unwrap() {
        match XCellTable::load_file(&excel, &config) {
            Success { value, diagnostics } => unity.write_class(&value)?,
            Failure { fatal, diagnostics } => {}
        }
    }
    Ok(())
}
