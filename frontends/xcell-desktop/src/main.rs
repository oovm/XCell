//! XCell GUI 主入口
//!
//! 初始化 Tauri 应用并设置命令和状态

use std::sync::Mutex;

use tauri::Manager;
use xcell_gui::{AppState, TauriLogger};

fn main() {
    TauriLogger::default().init().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            workspace: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler!(
            xcell_gui::read_table,
            xcell_gui::save_table,
            xcell_gui::validate_table,
            xcell_gui::generate_code,
            xcell_gui::get_table_list,
            xcell_gui::get_table_detail,
            xcell_gui::create_table,
            xcell_gui::update_table,
            xcell_gui::delete_table,
            xcell_gui::import_table,
            xcell_gui::export_table,
            xcell_gui::open_editor_window,
            xcell_gui::open_project_dialog,
            xcell_gui::open_project
        ))
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let setup_window = app.get_webview_window("setup").unwrap();
                setup_window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
