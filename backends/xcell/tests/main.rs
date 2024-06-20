use xcell::logger;
use xcell_core::{WorkspaceManager, XResult};

#[test]
fn ready() {
    println!("it works!")
}

#[ignore]
#[tokio::test]
async fn test_project_a() -> XResult {
    logger();
    let mut ws = WorkspaceManager::new("C:\\P4Root\\project\\OtherPlanet\\DataTables")?;
    ws.first_walk().await?;
    Ok(())
}
