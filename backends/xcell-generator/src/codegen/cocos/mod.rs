use xcell_analyzer::WorkspaceManager;
use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use xcell_types::{XError, XResult, XCellValue, for_3rd::ToPrimitive};
use xcell_analyzer::{XClassData, XListData, XDictData};
use url::Url;
use dejavu_macros::Template;

mod config;

/// 枚举项数据结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CocosEnumerateItem {
    /// 枚举键
    pub key: String,
    /// 枚举ID
    pub id: u32,
    /// 枚举名称
    pub name: String,
    /// 枚举描述
    pub description: String,
}

/// 字段数据结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CocosField {
    /// 字段名
    pub name: String,
    /// 字段类型
    pub r#type: String,
}

/// 数据表项数据结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CocosDataTableItem {
    /// 类名
    pub class_name: String,
    /// 表名
    pub table_name: String,
    /// 缓存名称
    pub cache_name: String,
    /// 获取方法名
    pub get_method_name: String,
}

/// Cocos 枚举模板
#[derive(Template)]
#[template(path = "CocosEnumerate.ts.dejavu")]
pub struct CocosEnumerateTemplate {
    /// 类名
    class_name: String,
    /// 枚举项
    items: Vec<CocosEnumerateItem>,
}

/// Cocos 类模板
#[derive(Template)]
#[template(path = "CocosClass.ts.dejavu")]
pub struct CocosClassTemplate {
    /// 类名
    class_name: String,
    /// 表名
    table_name: String,
    /// 字段
    fields: Vec<CocosField>,
    /// 是否有类型字段
    has_type_field: bool,
    /// 是否是怪物
    is_monster: bool,
}

/// Cocos 管理器模板
#[derive(Template)]
#[template(path = "CocosDataTableManager.ts.dejavu")]
pub struct CocosManagerTemplate {
    /// 表数据
    tables: Vec<CocosDataTableItem>,
    /// 表数据路径
    table_data_path: String,
}

// 使用dejavu模板生成代码
fn render_enumerate_template(class_name: &str, items: &[CocosEnumerateItem]) -> XResult<String> {
    let template = CocosEnumerateTemplate {
        class_name: class_name.to_string(),
        items: items.to_vec(),
    };
    template.render().map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
}

fn render_class_template(class_name: &str, table_name: &str, fields: &[CocosField], has_type_field: bool, is_monster: bool) -> XResult<String> {
    let template = CocosClassTemplate {
        class_name: class_name.to_string(),
        table_name: table_name.to_string(),
        fields: fields.to_vec(),
        has_type_field,
        is_monster,
    };
    template.render().map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
}

fn render_manager_template(tables: &[CocosDataTableItem], table_data_path: &str) -> XResult<String> {
    let template = CocosManagerTemplate {
        tables: tables.to_vec(),
        table_data_path: table_data_path.to_string(),
    };
    template.render().map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
}

/// Cocos 存储格式配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CocosStorage {
    /// JSON 存储配置
    pub json: CocosJsonConfig,
}

/// Cocos 加载器配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CocosLoader {
    /// 是否要生成 cocos 代码
    pub enable: bool,
    /// cocos 的工作目录, 建议使用相对路径
    pub project: String,
    /// 输出目录
    pub output: String,
    /// 生成的代码的命名空间
    pub namespace: String,
    /// 生成的管理器的名称
    pub manager_name: String,
    /// 生成的表格名的后缀
    pub suffix_table: String,
    /// 生成的实例名称
    pub instance_name: String,
    /// 表数据路径前缀
    pub table_data_path: String,
}

/// Cocos 代码生成器配置
///
/// 用于配置 Cocos 平台的代码生成
#[derive(Clone, Debug, Default, Serialize)]
pub struct CocosCodegen {
    /// 存储格式配置
    pub storage: CocosStorage,
    /// 是否要生成 cocos 代码
    pub enable: bool,
    /// cocos 的工作目录, 建议使用相对路径
    pub project: String,
    /// 输出目录
    pub output: String,
    /// 生成的代码的命名空间
    pub namespace: String,
    /// 生成的管理器的名称
    pub manager_name: String,
    /// 生成的表格名的后缀
    pub suffix_table: String,
    /// 生成的实例名称
    pub instance_name: String,
    /// 表数据路径前缀
    pub table_data_path: String,
}

/// Cocos JSON 配置
///
/// 用于配置 JSON 数据生成
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CocosJsonConfig {
    /// 是否启用 JSON 生成
    pub enable: bool,
    /// 生成的 JSON 文件目录
    pub output: String,
}

impl Default for CocosJsonConfig {
    fn default() -> Self {
        Self {
            enable: true,
            output: "tables".to_string(),
        }
    }
}

/// Cocos 代码生成器
///
/// 负责生成 Cocos 平台的代码和数据文件
impl CocosCodegen {
    /// 获取 Cocos 项目路径
    ///
    /// # 参数
    /// * `root` - 根目录路径
    ///
    /// # 返回值
    /// 返回 Cocos 项目的绝对路径，成功时返回 Ok(PathBuf)，失败时返回 XError。
    pub fn cocos_path(&self, root: &Path) -> XResult<PathBuf> {
        let cocos_config = self.to_xcell_config();
        let path = cocos_config.cocos_path(root)?;
        println!("Cocos project path: {:?}", path);
        Ok(path)
    }

    /// 获取 TypeScript 代码输出路径
    ///
    /// # 参数
    /// * `root` - 根目录路径
    /// * `file_name` - 文件名
    ///
    /// # 返回值
    /// 返回 TypeScript 文件的输出路径，成功时返回 Ok(PathBuf)，失败时返回 XError。
    pub fn cocos_typescript_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let cocos_config = self.to_xcell_config();
        let path = cocos_config.cocos_typescript_path(root, file_name)?;
        println!("Cocos TypeScript path: {:?}", path);
        Ok(path)
    }

    /// 获取 JSON 输出路径
    ///
    /// # 参数
    /// * `root` - 根目录路径
    /// * `file_name` - 文件名
    ///
    /// # 返回值
    /// 返回 JSON 文件的输出路径，成功时返回 Ok(PathBuf)，失败时返回 XError。
    pub fn cocos_json_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let cocos_config = self.to_xcell_config();
        let path = cocos_config.cocos_json_path(root, file_name)?;
        println!("Cocos JSON path: {:?}", path);
        Ok(path)
    }

    /// 转换为 xcell-config 中的 CocosCodegen 类型
    ///
    /// # 返回值
    /// 返回 xcell-config 中的 CocosCodegen 实例
    fn to_xcell_config(&self) -> xcell_config::cocos::CocosCodegen {
        xcell_config::cocos::CocosCodegen {
            enable: self.enable,
            project: self.project.clone(),
            output: self.output.clone(),
            manager_name: self.manager_name.clone(),
            suffix_table: self.suffix_table.clone(),
            instance_name: self.instance_name.clone(),
            table_data_path: self.table_data_path.clone(),
            storage: xcell_config::cocos::CocosStorage::Json(xcell_config::cocos::CocosJsonConfig {
                enable: self.storage.json.enable,
                output: self.storage.json.output.clone(),
            }),
            storage_debug: None,
        }
    }

    /// 获取管理器路径
    ///
    /// # 参数
    /// * `root` - 根目录路径
    ///
    /// # 返回值
    /// 返回管理器文件的输出路径，成功时返回 Ok(PathBuf)，失败时返回 XError。
    pub fn cocos_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.cocos_typescript_path(root, &self.manager_name)
    }

    /// 获取 TypeScript 相对路径
    ///
    /// # 参数
    /// * `file_name` - 文件名
    ///
    /// # 返回值
    /// 返回 TypeScript 文件的相对路径。
    pub fn cocos_ts_relative(&self, file_name: &str) -> String {
        format!("{}/{}.ts", self.output, file_name)
    }

    /// 获取 JSON 相对路径
    ///
    /// # 参数
    /// * `file_name` - 文件名
    ///
    /// # 返回值
    /// 返回 JSON 文件的相对路径。
    pub fn cocos_json_relative(&self, file_name: &str) -> String {
        format!("{}/{}.json", self.storage.json.output, file_name)
    }

    /// 确保输出目录存在
    ///
    /// # 参数
    /// * `root` - 根目录路径
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.cocos_typescript_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
            if self.storage.json.enable {
                if let Some(s) = self.cocos_json_path(root, "test")?.parent() {
                    std::fs::create_dir_all(s)?;
                }
            }
        }
        Ok(())
    }

    /// 写入 TypeScript 代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    pub fn write_typescript(&self, ws: &WorkspaceManager) -> XResult<()> {
        println!("CocosCodegen::write_typescript called");
        
        let root = &ws.config.root;
        
        if let Some(s) = self.cocos_typescript_path(root, "DataTableManager")?.parent() {
            std::fs::create_dir_all(s)?;
        }
        
        let csv_files = std::fs::read_dir(root)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension().map(|ext| ext == "csv").unwrap_or(false)
            })
            .collect::<Vec<_>>();
        
        for entry in &csv_files {
            let file_name_os = entry.file_name();
            let file_name_str = file_name_os.to_string_lossy();
            let file_name = file_name_str.to_string();
            let class_name = file_name.split('.').next().unwrap_or(&file_name);
            
            let is_enum = class_name.ends_with("Type") || class_name.ends_with("Kind");
            
            if is_enum {
                let ts_path = self.cocos_typescript_path(root, class_name)?;
                
                println!("Processing enum: {} -> {}", class_name, ts_path.display());
                
                if let Some(parent) = ts_path.parent() {
                    std::fs::create_dir_all(parent)?;
                    println!("Created directory: {:?}", parent);
                }
                
                let enum_data = self.read_enum_data(&entry.path())?;
                
                let items = enum_data.into_iter()
                    .map(|(id, name, description)| {
                        let key = name.to_uppercase().replace(" ", "_");
                        CocosEnumerateItem {
                            key,
                            id,
                            name,
                            description,
                        }
                    })
                    .collect::<Vec<_>>();
                
                let content = render_enumerate_template(class_name, &items)?;
                
                let mut file = File::create(ts_path)?;
                file.write_all(content.as_bytes())?;
                
                println!("Created TypeScript enum for {} successfully", class_name);
            } else {
                let table_class_name = format!("{}Table", class_name);
                let ts_path = self.cocos_typescript_path(root, &table_class_name)?;
                
                println!("Processing list table: {} -> {}", class_name, ts_path.display());
                
                if let Some(parent) = ts_path.parent() {
                    std::fs::create_dir_all(parent)?;
                    println!("Created directory: {:?}", parent);
                }
                
                let fields = self.read_csv_fields(&entry.path(), class_name)?;
                let has_type_field = fields.iter().any(|f| f.name == "type");
                
                let content = render_class_template(class_name, &table_class_name, &fields, has_type_field, class_name == "Monster")?;
                
                let mut file = File::create(ts_path)?;
                file.write_all(content.as_bytes())?;
                
                println!("Created TypeScript file for {} successfully", class_name);
            }
        }
        
        self.write_data_table_manager(ws)?;
        
        Ok(())
    }
    
    /// 读取 CSV 文件的字段信息
    ///
    /// # 参数
    /// * `csv_path` - CSV 文件路径
    /// * `class_name` - 类名
    ///
    /// # 返回值
    /// 返回字段信息列表，成功时返回 Ok(Vec<CocosField>)，失败时返回 XError。
    pub fn read_csv_fields(&self, csv_path: &Path, class_name: &str) -> XResult<Vec<CocosField>> {
        let mut fields = Vec::new();
        
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(csv_path)
            .map_err(|e| XError::runtime_error(format!("CSV read error: {}", e)))?;
        
        let records: Vec<csv::StringRecord> = rdr.records()
            .filter_map(|r| r.ok())
            .collect();
        
        if records.len() >= 2 {
            let headers = &records[0];
            let type_row = &records[1];
            
            for (i, header) in headers.iter().enumerate() {
                if i < type_row.len() {
                    let field_type = &type_row[i];
                    let ts_type = self.map_csv_type_to_typescript(field_type, header, class_name);
                    fields.push(CocosField {
                        name: header.to_string(),
                        r#type: ts_type,
                    });
                }
            }
        }
        
        Ok(fields)
    }
    
    /// 读取枚举类型的 CSV 文件数据
    ///
    /// # 参数
    /// * `csv_path` - CSV 文件路径
    ///
    /// # 返回值
    /// 返回枚举数据列表，每个元素包含 id、name 和 description，成功时返回 Ok(Vec<(u32, String, String)>)，失败时返回 XError。
    pub fn read_enum_data(&self, csv_path: &Path) -> XResult<Vec<(u32, String, String)>> {
        let mut enum_data = Vec::new();
        
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(csv_path)
            .map_err(|e| XError::runtime_error(format!("CSV read error: {}", e)))?;
        
        let records: Vec<csv::StringRecord> = rdr.records()
            .filter_map(|r| r.ok())
            .collect();
        
        if records.len() >= 3 {
            let headers = &records[0];
            
            let id_index = headers.iter().position(|h| config::is_id_field(h)).unwrap_or(0);
            let name_index = headers.iter().position(|h| config::is_name_field(h)).unwrap_or(1);
            let desc_index = headers.iter().position(|h| config::is_description_field(h)).unwrap_or_else(|| {
                name_index
            });
            
            for record in records.iter().skip(2) {
                if record.len() > id_index && record.len() > name_index && record.len() > desc_index {
                    if let Ok(id) = record[id_index].parse::<u32>() {
                        let name = record[name_index].to_string();
                        let description = record[desc_index].to_string();
                        enum_data.push((id, name, description));
                    }
                }
            }
        }
        
        Ok(enum_data)
    }
    
    /// 将 CSV 类型映射为 TypeScript 类型
    ///
    /// # 参数
    /// * `csv_type` - CSV 中的类型字符串
    /// * `field_name` - 字段名
    /// * `class_name` - 类名
    ///
    /// # 返回值
    /// 返回对应的 TypeScript 类型字符串。
    pub fn map_csv_type_to_typescript(&self, csv_type: &str, field_name: &str, class_name: &str) -> String {
        let trimmed_type = csv_type.trim();
        match trimmed_type {
            "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),
            "text" | "string" => {
                if field_name == "drop_items" || field_name == "skills" || field_name == "unlock_skills" {
                    "number[]".to_string()
                } else if field_name == "type" && class_name == "Monster" {
                    "MonsterType".to_string()
                } else {
                    "string".to_string()
                }
            }
            "string[]" => "number[]".to_string(),
            "any" => "string".to_string(),
            _ => config::get_type_mapping(trimmed_type).to_string(),
        }
    }
    
    /// 写入 DataTableManager.ts
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    pub fn write_data_table_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
        let root = &ws.config.root;
        
        let csv_files = std::fs::read_dir(root)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension().map(|ext| ext == "csv").unwrap_or(false)
            })
            .collect::<Vec<_>>();
        
        let manager_path = self.cocos_typescript_path(root, "DataTableManager")?;
        
        println!("Generating DataTableManager -> {}", manager_path.display());
        
        if let Some(parent) = manager_path.parent() {
            std::fs::create_dir_all(parent)?;
            println!("Created directory: {:?}", parent);
        }
        
        let table_data_path = if self.table_data_path.is_empty() { "tables/" } else { &self.table_data_path };
        
        let mut tables = Vec::new();
        
        for entry in &csv_files {
            let file_name_os = entry.file_name();
            let file_name_str = file_name_os.to_string_lossy();
            let file_name = file_name_str.to_string();
            let class_name = file_name.split('.').next().unwrap_or(&file_name);
            
            let is_enum = class_name.ends_with("Type") || class_name.ends_with("Kind");
            
            if !is_enum {
                let table_class_name = format!("{}Table", class_name);
                let cache_name = format!("{}Table", class_name.to_lowercase());
                let get_method_name = format!("get{}Table", class_name);
                
                tables.push(CocosDataTableItem {
                    class_name: class_name.to_string(),
                    table_name: table_class_name,
                    cache_name,
                    get_method_name,
                });
            }
        }
        
        let content = render_manager_template(&tables, table_data_path)?;
        
        let mut file = File::create(manager_path)?;
        file.write_all(content.as_bytes())?;
        
        println!("Created DataTableManager.ts successfully");
        
        Ok(())
    }

    /// 写入 JSON 数据
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    pub fn write_json(&self, ws: &WorkspaceManager) -> XResult<()> {
        if !self.enable || !self.storage.json.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

        let root = &ws.config.root;
        
        // 处理类表
        for table in ws.classes() {
            let json_path = self.cocos_json_path(root, &table.name)?;
            
            println!("Generating JSON for {} -> {}", table.name, json_path.display());
            
            if let Some(parent) = json_path.parent() {
                std::fs::create_dir_all(parent)?;
                println!("Created directory: {:?}", parent);
            }
            
            let json_data = self.convert_class_data_to_json(table)?;
            let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
            
            let mut file = std::fs::File::create(json_path)?;
            file.write_all(json_string.as_bytes())?;
            
            println!("Created JSON file for {} successfully", table.name);
        }
        
        // 处理列表
        for table in ws.lists() {
            let json_path = self.cocos_json_path(root, &table.name)?;
            
            println!("Generating JSON for {} -> {}", table.name, json_path.display());
            
            if let Some(parent) = json_path.parent() {
                std::fs::create_dir_all(parent)?;
                println!("Created directory: {:?}", parent);
            }
            
            let json_data = self.convert_list_data_to_json(table)?;
            let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
            
            let mut file = std::fs::File::create(json_path)?;
            file.write_all(json_string.as_bytes())?;
            
            println!("Created JSON file for {} successfully", table.name);
        }
        
        // 处理字典
        for table in ws.dicts() {
            let json_path = self.cocos_json_path(root, &table.name)?;
            
            println!("Generating JSON for {} -> {}", table.name, json_path.display());
            
            if let Some(parent) = json_path.parent() {
                std::fs::create_dir_all(parent)?;
                println!("Created directory: {:?}", parent);
            }
            
            let json_data = self.convert_dict_data_to_json(table)?;
            let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
            
            let mut file = std::fs::File::create(json_path)?;
            file.write_all(json_string.as_bytes())?;
            
            println!("Created JSON file for {} successfully", table.name);
        }

        Ok(())
    }
    
    /// 将类表数据转换为 JSON
    ///
    /// # 参数
    /// * `table` - 类表数据
    ///
    /// # 返回值
    /// 返回 JSON 数据，成功时返回 Ok(serde_json::Value)，失败时返回 XError。
    fn convert_class_data_to_json(&self, table: &XClassData) -> XResult<serde_json::Value> {
        let mut records = Vec::new();
        
        for item in &table.items {
            let mut record_map = serde_json::Map::new();
            record_map.insert("field".to_string(), serde_json::Value::String(item.field.clone()));
            record_map.insert("default".to_string(), self.convert_xcell_value_to_json(&item.default));
            
            records.push(serde_json::Value::Object(record_map));
        }
        
        Ok(serde_json::Value::Array(records))
    }
    
    /// 将列表数据转换为 JSON
    ///
    /// # 参数
    /// * `table` - 列表数据
    ///
    /// # 返回值
    /// 返回 JSON 数据，成功时返回 Ok(serde_json::Value)，失败时返回 XError。
    fn convert_list_data_to_json(&self, table: &XListData) -> XResult<serde_json::Value> {
        let mut records = Vec::new();
        
        for line in table.mapping.values() {
            let mut record_map = serde_json::Map::new();
            record_map.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(line.id.to_u64().unwrap_or(0))));
            record_map.insert("key".to_string(), serde_json::Value::String(line.key.clone()));
            
            // 处理数据字段
            for (i, header) in table.headers.iter().enumerate() {
                if i < line.data.len() {
                    let json_value = self.convert_xcell_value_to_json(&line.data[i]);
                    record_map.insert(header.field_name.clone(), json_value);
                }
            }
            
            records.push(serde_json::Value::Object(record_map));
        }
        
        Ok(serde_json::Value::Array(records))
    }
    
    /// 将字典数据转换为 JSON
    ///
    /// # 参数
    /// * `table` - 字典数据
    ///
    /// # 返回值
    /// 返回 JSON 数据，成功时返回 Ok(serde_json::Value)，失败时返回 XError。
    fn convert_dict_data_to_json(&self, table: &XDictData) -> XResult<serde_json::Value> {
        let mut records = Vec::new();
        
        for line in table.mapping.values() {
            let mut record_map = serde_json::Map::new();
            record_map.insert("id".to_string(), serde_json::Value::Number(serde_json::Number::from(line.id.to_u64().unwrap_or(0))));
            record_map.insert("key".to_string(), serde_json::Value::String(line.key.clone()));
            
            // 处理数据字段
            for (i, header) in table.headers.iter().enumerate() {
                if i < line.data.len() {
                    let json_value = self.convert_xcell_value_to_json(&line.data[i]);
                    record_map.insert(header.field_name.clone(), json_value);
                }
            }
            
            records.push(serde_json::Value::Object(record_map));
        }
        
        Ok(serde_json::Value::Array(records))
    }
    
    /// 将 XCellValue 转换为 JSON 值
    ///
    /// # 参数
    /// * `value` - XCellValue 值
    ///
    /// # 返回值
    /// 返回对应的 JSON 值
    fn convert_xcell_value_to_json(&self, value: &XCellValue) -> serde_json::Value {
        match value {
            XCellValue::Boolean(b) => serde_json::Value::Bool(*b),
            XCellValue::Integer8(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            XCellValue::Integer16(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            XCellValue::Integer32(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            XCellValue::Integer64(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            XCellValue::Unsigned8(u) => serde_json::Value::Number(serde_json::Number::from(*u)),
            XCellValue::Unsigned16(u) => serde_json::Value::Number(serde_json::Number::from(*u)),
            XCellValue::Unsigned32(u) => serde_json::Value::Number(serde_json::Number::from(*u)),
            XCellValue::Unsigned64(u) => serde_json::Value::Number(serde_json::Number::from(*u)),
            XCellValue::Float32(f) => serde_json::Value::Number(serde_json::Number::from_f64(*f as f64).unwrap()),
            XCellValue::Float64(f) => serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap()),
            XCellValue::String(s) => serde_json::Value::String(s.clone()),
            XCellValue::Vector2(v) => serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from_f64(v[0] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[1] as f64).unwrap())
            ]),
            XCellValue::Vector3(v) => serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from_f64(v[0] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[1] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[2] as f64).unwrap())
            ]),
            XCellValue::Vector4(v) => serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from_f64(v[0] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[1] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[2] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[3] as f64).unwrap())
            ]),
            XCellValue::Quaternion4(v) => serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from_f64(v[0] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[1] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[2] as f64).unwrap()),
                serde_json::Value::Number(serde_json::Number::from_f64(v[3] as f64).unwrap())
            ]),
            XCellValue::Color(c) => serde_json::Value::Object(serde_json::Map::from_iter(vec![
                ("r".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(c.r as f64).unwrap())),
                ("g".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(c.g as f64).unwrap())),
                ("b".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(c.b as f64).unwrap())),
                ("a".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(c.a as f64).unwrap()))
            ])),
            XCellValue::Vector(v) => serde_json::Value::Array(
                v.iter().map(|item| self.convert_xcell_value_to_json(item)).collect()
            ),
            XCellValue::Enumerate(s) => serde_json::Value::String(s.clone()),
            _ => serde_json::Value::Null,
        }
    }
    


    /// 记录 TypeScript 文件创建
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `name` - 文件名
    ///
    /// # 返回值
    /// 返回文件句柄，成功时返回 Ok(File)，失败时返回 XError。
    fn log_typescript(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.cocos_typescript_path(&ws.config.root, name)?;
        tracing::info!("写入 TypeScript: {}\n{}", self.cocos_ts_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }

    /// 记录 JSON 文件创建
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `name` - 文件名
    ///
    /// # 返回值
    /// 返回文件句柄，成功时返回 Ok(File)，失败时返回 XError。
    fn log_json(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.cocos_json_path(&ws.config.root, name)?;
        tracing::info!("写入 JSON: {}\n{}", self.cocos_json_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

impl super::Codegen for CocosCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        println!("CocosCodegen::generate called");
        println!("Output directory: {:?}", context.output_dir);
        
        if let Some(workspace) = &context.workspace {
            println!("Workspace root: {:?}", workspace.config.root);
            
            for generator in &workspace.config.generators {
                if let xcell_config::project::Generator::Cocos(cocos_config) = generator {
                    let cocos_codegen = CocosCodegen {
                        storage: match &cocos_config.storage {
                            xcell_config::cocos::CocosStorage::Json(config) => CocosStorage {
                                json: CocosJsonConfig {
                                    enable: config.enable,
                                    output: config.output.clone(),
                                },
                            },
                        },
                        enable: cocos_config.enable,
                        project: cocos_config.project.clone(),
                        output: cocos_config.output.clone(),
                        namespace: String::new(),
                        manager_name: cocos_config.manager_name.clone(),
                        suffix_table: cocos_config.suffix_table.clone(),
                        instance_name: cocos_config.instance_name.clone(),
                        table_data_path: cocos_config.table_data_path.clone(),
                    };
                    
                    println!("Cocos codegen enable: {}", cocos_codegen.enable);
                    println!("Cocos project: {}", cocos_codegen.project);
                    println!("Cocos output: {}", cocos_codegen.output);
                    
                    println!("Calling write_typescript");
                    cocos_codegen.write_typescript(workspace)?;
                    println!("write_typescript completed");
                    println!("Calling write_json");
                    cocos_codegen.write_json(workspace)?;
                    println!("write_json completed");
                }
            }
        } else {
            println!("No workspace manager in context");
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "cocos"
    }
}
