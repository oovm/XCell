#![warn(missing_docs)]

//! XCell GUI Tauri 应用库
//!
//! 提供 Tauri 命令和状态管理

mod commands;
mod errors;
mod logger;

pub use commands::*;
pub use logger::TauriLogger;
