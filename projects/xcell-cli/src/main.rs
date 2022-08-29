use std::{env::current_dir, path::PathBuf, process::Command};

use clap::{Parser, Subcommand};

use xcell_core::{WorkspaceManager, XResult};

use crate::utils::{logger, pause};

mod utils;
mod workspace;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 手动设置工作目录, 无表示当前目录
    #[arg(short, long, default_value_t = String::new())]
    workspace: String,
    #[command(subcommand)]
    command: SubArgs,
}

#[derive(Subcommand, Debug)]
pub enum SubArgs {
    /// Clear database and cache
    Clear,
}

#[tokio::main]
async fn main() -> XResult {
    logger();
    let args = Args::parse();
    let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
    ws.first_walk().await?;
    pause()
}
