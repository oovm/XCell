//! Tauri 命令模块
//!
//! 提供前端可调用的 Tauri 命令

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use log::{error, info};
use serde_json::Value;
use xcell_analyzer::WorkspaceManager;

/// 应用状态
pub struct AppState {
    /// 工作空间管理器
    pub workspace: Mutex<Option<WorkspaceManager>>,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

/// 读取表格文件
#[tauri::command]
pub async fn read_table(
    state: tauri::State<'_, AppState>,
    file_path: String,
) -> Result<Value, String> {
    info!("Reading table from file: {}", file_path);

    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let workspace_path = match path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => return Err("Invalid file path".to_string()),
    };

    let mut workspace = WorkspaceManager::new(workspace_path)
        .map_err(|e| format!("Failed to create workspace: {}", e))?;

    workspace
        .first_walk()
        .map_err(|e| format!("Failed to walk workspace: {}", e))?;

    let table_name = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let table_data = lookup_table_data(&workspace, &table_name)?;

    let mut guard = state.workspace.lock().map_err(|e| format!("Lock error: {}", e))?;
    *guard = Some(workspace);

    Ok(table_data)
}

/// 保存表格数据
#[tauri::command]
pub async fn save_table(_file_path: String, _data: Value) -> Result<bool, String> {
    info!("Save table called (not yet implemented)");
    Err("Save functionality is not yet implemented at the backend level".to_string())
}

/// 验证表格文件
#[tauri::command]
pub async fn validate_table(file_path: String) -> Result<Value, String> {
    info!("Validating table file: {}", file_path);

    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let workspace_path = match path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => return Err("Invalid file path".to_string()),
    };

    let mut workspace = WorkspaceManager::new(workspace_path)
        .map_err(|e| format!("Failed to create workspace: {}", e))?;

    match workspace.try_perform_file(&path) {
        Ok(_) => Ok(serde_json::json!({
            "valid": true,
            "errors": []
        })),
        Err(e) => Ok(serde_json::json!({
            "valid": false,
            "errors": [e.to_string()]
        })),
    }
}

/// 生成代码
#[tauri::command]
pub async fn generate_code(
    state: tauri::State<'_, AppState>,
    workspace_path: String,
    target: String,
) -> Result<bool, String> {
    info!(
        "Generating code for workspace: {}, target: {}",
        workspace_path, target
    );

    let path = PathBuf::from(&workspace_path);
    if !path.exists() || !path.is_dir() {
        return Err(format!("Invalid workspace path: {}", workspace_path));
    }

    let mut workspace = WorkspaceManager::new(path)
        .map_err(|e| format!("Failed to create workspace: {}", e))?;

    workspace
        .first_walk()
        .map_err(|e| format!("Failed to load workspace: {}", e))?;

    match target.as_str() {
        "unity" => workspace
            .write_unity()
            .map_err(|e| format!("Failed to generate Unity code: {}", e)),
        "cocos" => workspace
            .write_cocos()
            .map_err(|e| format!("Failed to generate Cocos code: {}", e)),
        _ => return Err(format!("Unsupported target: {}", target)),
    }?;

    let mut guard = state.workspace.lock().map_err(|e| format!("Lock error: {}", e))?;
    *guard = Some(workspace);

    Ok(true)
}

/// 获取表格列表
#[tauri::command]
pub async fn get_table_list(
    state: tauri::State<'_, AppState>,
) -> Result<Value, String> {
    info!("Getting table list");

    let guard = state.workspace.lock().map_err(|e| format!("Lock error: {}", e))?;
    let workspace = match guard.as_ref() {
        Some(ws) => ws,
        None => return Err("No workspace is currently open".to_string()),
    };

    let mut table_list = Vec::new();

    let first_mtime = workspace
        .file_modification_times
        .values()
        .next()
        .copied();

    for list in workspace.lists() {
        table_list.push(serde_json::json!({
            "id": list.name,
            "name": list.name,
            "type": "table",
            "path": format!("tables/{}.xlsx", list.name),
            "draft": false,
            "createdAt": format_system_time(first_mtime)
        }));
    }

    for dict in workspace.dicts() {
        table_list.push(serde_json::json!({
            "id": dict.name,
            "name": dict.name,
            "type": "table",
            "path": format!("tables/{}.xlsx", dict.name),
            "draft": false,
            "createdAt": format_system_time(first_mtime)
        }));
    }

    for class in workspace.classes() {
        table_list.push(serde_json::json!({
            "id": class.name,
            "name": class.name,
            "type": "class",
            "path": format!("classes/{}.xlsx", class.name),
            "draft": false,
            "createdAt": format_system_time(first_mtime)
        }));
    }

    for enumerate in workspace.enumerates() {
        table_list.push(serde_json::json!({
            "id": enumerate.name,
            "name": enumerate.name,
            "type": "enum",
            "path": format!("enums/{}.xlsx", enumerate.name),
            "draft": false,
            "createdAt": format_system_time(first_mtime)
        }));
    }

    for language in workspace.languages() {
        table_list.push(serde_json::json!({
            "id": language.key,
            "name": language.key,
            "type": "language",
            "path": format!("languages/{}.xlsx", language.key),
            "draft": false,
            "createdAt": format_system_time(first_mtime)
        }));
    }

    Ok(Value::Array(table_list))
}

/// 获取表格详情
#[tauri::command]
pub async fn get_table_detail(
    state: tauri::State<'_, AppState>,
    table_id: String,
) -> Result<Value, String> {
    info!("Getting table detail for id: {}", table_id);

    let guard = state.workspace.lock().map_err(|e| format!("Lock error: {}", e))?;
    let workspace = match guard.as_ref() {
        Some(ws) => ws,
        None => return Err("No workspace is currently open".to_string()),
    };

    lookup_table_data(workspace, &table_id)
}

/// 创建表格
#[tauri::command]
pub async fn create_table(_table: Value) -> Result<Value, String> {
    info!("Create table called (not yet supported)");
    Err("Create table is not yet supported by the backend".to_string())
}

/// 更新表格
#[tauri::command]
pub async fn update_table(_table_id: String, _table: Value) -> Result<Value, String> {
    info!("Update table called (not yet supported)");
    Err("Update table is not yet supported by the backend".to_string())
}

/// 删除表格
#[tauri::command]
pub async fn delete_table(_table_id: String) -> Result<bool, String> {
    info!("Delete table called (not yet supported)");
    Err("Delete table is not yet supported by the backend".to_string())
}

/// 导入表格
#[tauri::command]
pub async fn import_table(_file_name: String, _file_content: String) -> Result<Value, String> {
    info!("Import table called (not yet supported)");
    Err("Import table is not yet supported by the backend".to_string())
}

/// 导出表格
#[tauri::command]
pub async fn export_table(_table_id: String) -> Result<String, String> {
    info!("Export table called (not yet supported)");
    Err("Export table is not yet supported by the backend".to_string())
}

/// 打开文件选择对话框并选择项目文件夹
#[tauri::command]
pub fn open_project_dialog(app: tauri::AppHandle) -> Result<String, String> {
    info!("Opening project selection dialog");

    use tauri_plugin_dialog::DialogExt;

    let folder_path = app.dialog().file().blocking_pick_folder();

    match folder_path {
        Some(path) => {
            let project_path = path.to_string();
            info!("Selected project path: {}", project_path);
            Ok(project_path)
        }
        None => {
            info!("User cancelled project selection");
            Err("User cancelled".to_string())
        }
    }
}

/// 打开编辑器窗口
#[tauri::command]
pub fn open_editor_window(
    app: tauri::AppHandle,
    project_path: String,
) -> Result<(), String> {
    info!("Opening editor window for project: {}", project_path);

    use rand::Rng;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let random = rand::thread_rng().gen_range(1000..9999);
    let window_label = format!("editor_{}_{}", timestamp, random);

    let project_name = project_path
        .split(std::path::MAIN_SEPARATOR)
        .last()
        .unwrap_or(&project_path);

    let url = format!(
        "/project?path={}",
        urlencoding::encode(&project_path)
    );

    let window = tauri::WebviewWindowBuilder::new(
        &app,
        &window_label,
        tauri::WebviewUrl::App(url.into()),
    )
    .title(format!("XCell - 编辑器 - {}", project_name))
    .inner_size(1200.0, 800.0)
    .resizable(true)
    .fullscreen(false)
    .center()
    .build()
    .map_err(|e| {
        error!("Failed to create window: {}", e);
        e.to_string()
    })?;

    window.show().map_err(|e| {
        error!("Failed to show window: {}", e);
        e.to_string()
    })?;

    window.set_focus().map_err(|e| {
        error!("Failed to set focus: {}", e);
        e.to_string()
    })?;

    info!("Successfully opened editor window: {}", window_label);
    Ok(())
}

/// 打开项目并加载工作空间
#[tauri::command]
pub async fn open_project(
    state: tauri::State<'_, AppState>,
    project_path: String,
) -> Result<bool, String> {
    info!("Opening project: {}", project_path);

    let path = PathBuf::from(&project_path);
    if !path.exists() || !path.is_dir() {
        return Err(format!("Invalid project path: {}", project_path));
    }

    let mut workspace = WorkspaceManager::new(path)
        .map_err(|e| format!("Failed to create workspace: {}", e))?;

    workspace
        .first_walk()
        .map_err(|e| format!("Failed to load workspace: {}", e))?;

    let mut guard = state.workspace.lock().map_err(|e| format!("Lock error: {}", e))?;
    *guard = Some(workspace);

    info!("Project opened successfully: {}", project_path);
    Ok(true)
}

fn lookup_table_data(workspace: &WorkspaceManager, table_name: &str) -> Result<Value, String> {
    if let Some(list) = workspace.get_list(table_name) {
        return serde_json::to_value(list).map_err(|e| format!("Serialization error: {}", e));
    }

    if let Some(dict) = workspace.get_dict(table_name) {
        return serde_json::to_value(dict).map_err(|e| format!("Serialization error: {}", e));
    }

    if let Some(enumerate) = workspace.get_enumerate(table_name) {
        return serde_json::to_value(enumerate).map_err(|e| format!("Serialization error: {}", e));
    }

    if let Some(class) = workspace.get_class(table_name) {
        return serde_json::to_value(class).map_err(|e| format!("Serialization error: {}", e));
    }

    if let Some(language) = workspace.get_language(table_name) {
        return serde_json::to_value(language).map_err(|e| format!("Serialization error: {}", e));
    }

    Err(format!("Table '{}' not found", table_name))
}

fn format_system_time(time: Option<SystemTime>) -> String {
    match time {
        Some(t) => {
            let duration = t.duration_since(UNIX_EPOCH).unwrap_or_default();
            let secs = duration.as_secs();
            chrono::DateTime::from_timestamp(secs as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default()
        }
        None => String::new(),
    }
}
