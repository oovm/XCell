#![warn(missing_docs)]
use std::{env::current_dir, path::PathBuf};

use clap::{Parser, Subcommand};

use xcell_analyzer::XResult;

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
    /// 编辑 TOML 配置文件
    Toml {
        #[command(subcommand)]
        subcommand: TomlSubArgs,
    },
}

#[derive(Subcommand, Debug)]
pub enum TomlSubArgs {
    /// 列出 TOML 文件中的字段
    List {
        /// TOML 文件路径
        file: String,
    },
    /// 添加字段到 TOML 文件
    Add {
        /// TOML 文件路径
        file: String,
        /// 字段名称
        name: String,
        /// 字段类型
        r#type: String,
        /// 字段注释
        #[arg(short, long)]
        comment: Option<String>,
        /// 字段默认值
        #[arg(short, long)]
        default: Option<String>,
    },
    /// 从 TOML 文件中删除字段
    Remove {
        /// TOML 文件路径
        file: String,
        /// 字段名称
        name: String,
    },
    /// 更新 TOML 文件中的字段
    Update {
        /// TOML 文件路径
        file: String,
        /// 字段名称
        name: String,
        /// 字段类型
        #[arg(short, long)]
        r#type: Option<String>,
        /// 字段注释
        #[arg(short, long)]
        comment: Option<String>,
        /// 字段默认值
        #[arg(short, long)]
        default: Option<String>,
    },
}
