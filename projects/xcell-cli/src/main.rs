use std::{env::current_dir, path::PathBuf, process::Command};

use clap::{Parser, Subcommand};

use xcell_core::{WorkspaceManager, XResult};

use crate::utils::{logger, pause};

mod utils;
mod workspace;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// 手动设置工作目录, 无表示当前目录
    #[arg(long, default_value_t = String::new())]
    workspace: String,
    /// 启用监听模式, 当有文件修改时只更新对应文件
    #[arg(short, long, default_value_t = false)]
    watch: bool,
    /// 强制关闭 xml 生成
    #[arg(long, default_value_t = false)]
    disable_xml: bool,
    /// 强制关闭 json 生成
    #[arg(long, default_value_t = false)]
    disable_json: bool,
    #[command(subcommand)]
    command: Option<SubArgs>,
}

#[derive(Subcommand, Debug)]
pub enum SubArgs {
    /// 检查配置表, 不导出任何文件
    Check,
    /// 清除数据库与缓存
    Clear,
}

#[tokio::main]
async fn main() -> XResult {
    logger();
    let args = Args::parse();
    let mut ws = WorkspaceManager::new(args.resolve_workspace()?)?;
    if args.disable_xml {
        ws.disable_xml()
    }
    if args.disable_json {
        ws.disable_json()
    }
    ws.first_walk().await?;
    pause();
    Ok(())
}
