#![warn(missing_docs)]

//! XCell GUI Tauri 应用库
//!
//! 提供 Tauri 命令和状态管理

mod commands;
mod errors;
mod logger;

pub use commands::{
    AppState, create_table, delete_table, export_table, generate_code, get_table_detail,
    get_table_list, import_table, open_editor_window, open_project, open_project_dialog,
    read_table, save_table, update_table, validate_table,
};
pub use logger::TauriLogger;
