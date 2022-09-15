use std::{env::current_dir, path::PathBuf, process::Command};

use clap::{Parser, Subcommand};

use xcell_core::XResult;

pub use crate::utils::{logger, pause};

mod utils;
mod workspace;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct XCellArgs {
    /// 手动设置工作目录, 无表示当前目录
    #[arg(long, default_value_t = String::new())]
    pub workspace: String,
    /// 启用监听模式, 当有文件修改时只更新对应文件
    #[arg(short, long, default_value_t = false)]
    pub watch: bool,
    /// 强制关闭 xml 生成
    #[arg(long, default_value_t = false)]
    pub disable_xml: bool,
    /// 强制关闭 json 生成
    #[arg(long, default_value_t = false)]
    pub disable_json: bool,
    #[command(subcommand)]
    pub command: Option<SubArgs>,
}

#[derive(Subcommand, Debug)]
pub enum SubArgs {
    /// 检查配置表, 不导出任何文件
    Check,
    /// 清除数据库与缓存
    Clear,
}
