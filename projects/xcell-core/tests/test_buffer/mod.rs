#![allow(unused)]

use xcell_core::{WorkspaceManager, XResult};

use super::*;

#[ignore]
#[tokio::test]
async fn test_project_a() -> XResult {
    logger();
    let mut ws = WorkspaceManager::new("F:\\project-a\\table")?;
    // let mut ws = WorkspaceManager::new("C:\\P4Root\\project\\EndlessHunter\\DataTables")?;
    ws.first_walk().await;
    Ok(())
}
