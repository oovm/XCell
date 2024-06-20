//! Tauri 命令模块
//!
//! 提供前端可调用的 Tauri 命令

use log::{info, error};
use std::path::PathBuf;
use tauri::Manager;
use xcell_analyzer::{WorkspaceManager, XResult};
use xcell_types::XError;

/// 简单的计数器命令
#[tauri::command]
pub fn increment_counter(value: i32) -> i32 {
    let new_value = value + 1;
    info!("Counter incremented to: {}", new_value);
    new_value
}

/// 简单的递减器命令
#[tauri::command]
pub fn decrement_counter(value: i32) -> i32 {
    let new_value = value - 1;
    info!("Counter decremented to: {}", new_value);
    new_value
}

/// 读取表格文件
#[tauri::command]
pub async fn read_table(file_path: String) -> Result<serde_json::Value, String> {
    info!("Reading table from file: {}", file_path);

    let path = PathBuf::from(file_path.clone());
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    // 获取文件所在目录作为工作目录
    let workspace_path = match path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => return Err("Invalid file path".to_string()),
    };

    // 创建 WorkspaceManager
    let mut workspace = match WorkspaceManager::new(workspace_path) {
        Ok(workspace) => workspace,
        Err(e) => return Err(format!("Failed to create workspace: {}", e)),
    };

    // 加载文件
    workspace.load_file(&path);

    // 这里需要实现表格数据的序列化
    // 暂时返回一个示例 JSON
    let table_data = serde_json::json!({
        "name": path.file_name().unwrap_or_default().to_string_lossy(),
        "path": file_path,
        "columns": [
            { "prop": "id", "label": "ID", "width": 100, "editable": false },
            { "prop": "name", "label": "名称", "width": 200, "editable": true },
            { "prop": "value", "label": "值", "width": 300, "editable": true }
        ],
        "data": [
            { "id": "1", "name": "Item 1", "value": "Value 1" },
            { "id": "2", "name": "Item 2", "value": "Value 2" },
            { "id": "3", "name": "Item 3", "value": "Value 3" }
        ]
    });

    Ok(table_data)
}

/// 保存表格数据
#[tauri::command]
pub async fn save_table(file_path: String, _data: serde_json::Value) -> Result<bool, String> {
    info!("Saving table: {}", file_path);

    // 这里需要实现表格数据的保存逻辑
    // 暂时返回成功
    Ok(true)
}

/// 验证表格文件
#[tauri::command]
pub async fn validate_table(file_path: String) -> Result<serde_json::Value, String> {
    info!("Validating table file: {}", file_path);

    let path = PathBuf::from(file_path.clone());
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    // 获取文件所在目录作为工作目录
    let workspace_path = match path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => return Err("Invalid file path".to_string()),
    };

    // 创建 WorkspaceManager
    let mut workspace = match WorkspaceManager::new(workspace_path) {
        Ok(workspace) => workspace,
        Err(e) => return Err(format!("Failed to create workspace: {}", e)),
    };

    // 尝试执行文件验证
    match workspace.try_perform_file(&path) {
        Ok(_) => {
            // 验证成功
            let validation_result = serde_json::json!({
                "valid": true,
                "errors": []
            });
            Ok(validation_result)
        }
        Err(e) => {
            // 验证失败
            let validation_result = serde_json::json!({
                "valid": false,
                "errors": [e.to_string()]
            });
            Ok(validation_result)
        }
    }
}

/// 生成代码
#[tauri::command]
pub async fn generate_code(workspace_path: String, target: String) -> Result<bool, String> {
    info!("Generating code for workspace: {}, target: {}", workspace_path, target);

    let path = PathBuf::from(workspace_path.clone());
    if !path.exists() || !path.is_dir() {
        return Err(format!("Invalid workspace path: {}", workspace_path));
    }

    // 创建 WorkspaceManager
    let mut workspace = match WorkspaceManager::new(path) {
        Ok(workspace) => workspace,
        Err(e) => return Err(format!("Failed to create workspace: {}", e)),
    };

    // 首次加载目录
    match workspace.first_walk() {
        Ok(_) => {
            // 根据目标生成代码
            match target.as_str() {
                "unity" => match workspace.write_unity() {
                    Ok(_) => Ok(true),
                    Err(e) => Err(format!("Failed to generate Unity code: {}", e)),
                },
                "cocos" => match workspace.write_cocos() {
                    Ok(_) => Ok(true),
                    Err(e) => Err(format!("Failed to generate Cocos code: {}", e)),
                },
                _ => Err(format!("Unsupported target: {}", target)),
            }
        }
        Err(e) => Err(format!("Failed to load workspace: {}", e)),
    }
}

/// 获取表格列表
#[tauri::command]
pub async fn get_table_list() -> Result<serde_json::Value, String> {
    info!("Getting table list");

    // 这里需要实现获取表格列表的逻辑
    // 暂时返回示例数据
    let table_list = serde_json::json!([
        {
            "id": "1",
            "name": "ExampleTable",
            "type": "table",
            "path": "tables/ExampleTable.xlsx",
            "draft": false,
            "createdAt": "2024-01-01 10:00:00"
        },
        {
            "id": "2",
            "name": "UserClass",
            "type": "class",
            "path": "classes/UserClass.xlsx",
            "draft": false,
            "createdAt": "2024-01-02 14:30:00"
        },
        {
            "id": "3",
            "name": "StatusEnum",
            "type": "enum",
            "path": "enums/StatusEnum.xlsx",
            "draft": true,
            "createdAt": "2024-01-03 09:15:00"
        },
        {
            "id": "4",
            "name": "LanguagePack",
            "type": "language",
            "path": "languages/LanguagePack.xlsx",
            "draft": false,
            "createdAt": "2024-01-04 11:45:00"
        },
        {
            "id": "5",
            "name": "ProductTable",
            "type": "table",
            "path": "tables/ProductTable.xlsx",
            "draft": false,
            "createdAt": "2024-01-05 16:20:00"
        }
    ]);

    Ok(table_list)
}

/// 获取表格详情
#[tauri::command]
pub async fn get_table_detail(table_id: String) -> Result<serde_json::Value, String> {
    info!("Getting table detail for id: {}", table_id);

    // 这里需要实现获取表格详情的逻辑
    // 暂时返回示例数据
    let table_detail = serde_json::json!({
        "id": table_id,
        "name": format!("Table {}", table_id),
        "columns": [
            { "prop": "id", "label": "ID", "width": 100, "editable": false },
            { "prop": "name", "label": "名称", "width": 200, "editable": true },
            { "prop": "age", "label": "年龄", "width": 100, "editable": true },
            { "prop": "email", "label": "邮箱", "width": 300, "editable": true }
        ],
        "data": [
            { "id": "1", "name": "张三", "age": 25, "email": "zhangsan@example.com" },
            { "id": "2", "name": "李四", "age": 30, "email": "lisi@example.com" },
            { "id": "3", "name": "王五", "age": 35, "email": "wangwu@example.com" }
        ]
    });

    Ok(table_detail)
}

/// 创建表格
#[tauri::command]
pub async fn create_table(table: serde_json::Value) -> Result<serde_json::Value, String> {
    info!("Creating table: {:?}", table);

    // 这里需要实现创建表格的逻辑
    // 暂时返回示例数据
    use std::time::{SystemTime, UNIX_EPOCH};
    use chrono::Utc;
    
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
    let now = Utc::now().to_string();
    
    let mut result = table.as_object().unwrap().clone();
    result.insert("id".to_string(), serde_json::Value::String(timestamp));
    result.insert("createdAt".to_string(), serde_json::Value::String(now));

    Ok(serde_json::Value::Object(result))
}

/// 更新表格
#[tauri::command]
pub async fn update_table(table_id: String, table: serde_json::Value) -> Result<serde_json::Value, String> {
    info!("Updating table {}: {:?}", table_id, table);

    // 这里需要实现更新表格的逻辑
    // 暂时返回示例数据
    use chrono::Utc;
    
    let now = Utc::now().to_string();
    
    let result = serde_json::json!(
        {
            "id": table_id,
            "name": table.get("name").unwrap_or(&serde_json::Value::String("Unknown".to_string())).as_str().unwrap_or("Unknown"),
            "type": table.get("type").unwrap_or(&serde_json::Value::String("table".to_string())).as_str().unwrap_or("table"),
            "path": table.get("path").unwrap_or(&serde_json::Value::String("".to_string())).as_str().unwrap_or(""),
            "draft": table.get("draft").unwrap_or(&serde_json::Value::Bool(false)).as_bool().unwrap_or(false),
            "createdAt": table.get("createdAt").unwrap_or(&serde_json::Value::String(now.clone())).as_str().unwrap_or(&now)
        }
    );

    Ok(result)
}

/// 删除表格
#[tauri::command]
pub async fn delete_table(table_id: String) -> Result<bool, String> {
    info!("Deleting table: {}", table_id);

    // 这里需要实现删除表格的逻辑
    // 暂时返回成功
    Ok(true)
}

/// 导入表格
#[tauri::command]
pub async fn import_table(file_name: String, _file_content: String) -> Result<serde_json::Value, String> {
    info!("Importing table: {}", file_name);

    // 这里需要实现导入表格的逻辑
    // 暂时返回示例数据
    use std::time::{SystemTime, UNIX_EPOCH};
    use chrono::Utc;
    
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
    let now = Utc::now().to_string();
    
    // 移除文件扩展名
    let name = file_name.split('.').next().unwrap_or(&file_name).to_string();
    
    let result = serde_json::json!({
        "id": timestamp,
        "name": name,
        "type": "table",
        "path": format!("tables/{}", file_name),
        "draft": false,
        "createdAt": now
    });

    Ok(result)
}

/// 导出表格
#[tauri::command]
pub async fn export_table(table_id: String) -> Result<String, String> {
    info!("Exporting table: {}", table_id);

    // 这里需要实现导出表格的逻辑
    // 暂时返回示例数据
    let result = serde_json::json!({
        "id": table_id,
        "name": format!("Table {}", table_id),
        "data": [
            { "id": "1", "name": "张三", "age": 25, "email": "zhangsan@example.com" },
            { "id": "2", "name": "李四", "age": 30, "email": "lisi@example.com" },
            { "id": "3", "name": "王五", "age": 35, "email": "wangwu@example.com" }
        ]
    });

    Ok(result.to_string())
}

/// 打开文件选择对话框并选择项目文件夹
#[tauri::command]
pub fn open_project_dialog(app: tauri::AppHandle) -> Result<String, String> {
    info!("Opening project selection dialog");
    
    // 使用 tauri-plugin-dialog 2.0 API
    use tauri_plugin_dialog::{FileDialogBuilder, Dialog};
    
    // 获取 Dialog 实例
    let dialog = Dialog::default();
    
    // 创建文件对话框并选择文件夹
    let path = FileDialogBuilder::new(dialog)
        .set_title("选择项目文件夹")
        .pick_folder();
    
    // 处理对话框结果
    match path {
        Some(path) => {
            let project_path = path.to_string_lossy().to_string();
            info!("Selected project path: {}", project_path);
            Ok(project_path)
        },
        None => {
            info!("User cancelled project selection");
            Err("User cancelled".to_string())
        }
    }
}

/// 打开编辑器窗口
#[tauri::command]
pub fn open_editor_window(app: tauri::AppHandle, project_path: String) -> Result<(), String> {
    info!("Opening editor window for project: {}", project_path);

    // 生成唯一的窗口标签（使用时间戳和随机数确保唯一性）
    use std::time::{SystemTime, UNIX_EPOCH};
    use rand::Rng;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random = rand::thread_rng().gen_range(1000..9999);
    let window_label = format!("editor_{}_{}", timestamp, random);

    // 提取项目名称用于窗口标题
    let project_name = project_path
        .split(std::path::MAIN_SEPARATOR)
        .last()
        .unwrap_or(&project_path);

    // 创建带查询参数的 URL，将项目路径传递给新窗口
    let url = format!("/project?path={}", urlencoding::encode(&project_path));

    // 创建新的编辑器窗口
    let window = tauri::WebviewWindowBuilder::new(
        &app,
        &window_label,
        tauri::WebviewUrl::App(url.into())
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

    // 显示窗口
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
