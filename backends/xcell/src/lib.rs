#![warn(missing_docs)]
use std::{env::current_dir, path::PathBuf};

use clap::{Parser, Subcommand};

use xcell_analyzer::XResult;

pub use crate::utils::{logger, pause};

mod utils;
mod workspace;

/// XCell 配置表管理工具
#[derive(Parser, Debug)]
#[command(author, version, about = "XCell 配置表管理工具")]
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
    /// 启用详细日志输出
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    /// 静默模式，仅输出错误信息
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
    /// 试运行模式，仅分析不生成输出文件
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
    /// 通过 glob 模式过滤需要处理的表格文件
    #[arg(long, default_value_t = String::new())]
    pub filter: String,
    /// TOML 配置文件子命令
    #[command(subcommand)]
    pub command: Option<SubArgs>,
}

/// CLI 子命令参数
#[derive(Subcommand, Debug)]
pub enum SubArgs {
    /// 检查配置表, 不导出任何文件
    Check,
    /// 清除数据库与缓存
    Clear,
    /// 显示工作空间配置和状态摘要
    Info,
    /// 初始化工作空间，创建默认 ProjectConfig.toml
    Init,
    /// 列出工作空间中已加载的表格及其类型
    List,
    /// 编辑 TOML 配置文件
    Toml {
        #[command(subcommand)]
        subcommand: TomlSubArgs,
    },
}

/// TOML 配置文件编辑子命令
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
