#![allow(unused)]

use xcell_core::{WorkspaceManager, XResult};

use super::*;

#[tokio::test]
async fn test2() -> XResult {
    logger();
    let mut ws = WorkspaceManager::new("F:\\project-a\\table")?;
    ws.first_walk().await;
    Ok(())
}
