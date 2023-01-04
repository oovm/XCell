use clap::Parser;
use xcell::{logger, pause, SubArgs, XCellArgs};
use xcell_core::{WorkspaceManager, XResult};

#[tokio::main]
async fn main() -> XResult {
    logger();
    let args = XCellArgs::parse();
    let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
    match args.command {
        Some(SubArgs::Check) => ws.dry_run(),
        Some(SubArgs::Clear) => {
            ws.clear()?;
            return Ok(());
        }
        _ => {}
    }
    if args.disable_xml {
        ws.disable_xml()
    }
    if args.disable_json {
        ws.disable_json()
    }
    ws.first_walk().await?;
    if args.watch {
        ws.watcher().await?
    }
    pause();
    Ok(())
}
