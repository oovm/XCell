use xcell_analyzer::WorkspaceManager;
use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use xcell_core::{XError, XResult, XCellValue, for_3rd::ToPrimitive};
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

/// 枚举键值对
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    /// 键
    pub key: String,
    /// 值
    pub value: String,
    /// 文档
    pub document: Vec<String>,
}

/// Cocos 枚举模板
#[derive(Template)]
#[template(path = "BuildEnumerate.ts.dejavu")]
pub struct CocosEnumerateTemplate {
    /// 编译器版本
    compiler_version: &'static str,
    /// 类名
    class_name: String,
    /// ID 类型
    id_type: String,
    /// Cocos 代码生成配置
    config: CocosCodegen,
    /// 枚举 ID 列表
    enumerate_ids: Vec<EnumeratePair>,
    /// 类文档
    class_document: Vec<String>,
}

/// Cocos 类模板
#[derive(Template)]
#[template(path = "BuildClass.ts.dejavu", escape = "none")]
pub struct CocosClassTemplate {
    /// 编译器版本
    compiler_version: &'static str,
    /// 类名
    class_name: String,
    /// 表名
    table_name: String,
    /// ID 类型
    id_type: String,
    /// Cocos 代码生成配置
    config: CocosCodegen,
    /// 键名
    key_name: String,
    /// 类文档
    class_document: Vec<String>,
    /// 类字段
    class_fields: Vec<ClassFieldTemplate>,
}

/// 类字段模板数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassFieldTemplate {
    /// 字段文档
    pub document: Vec<String>,
    /// 字段名
    pub name: String,
    /// 字段类型
    pub typing: String,
    /// 是否有默认值
    pub has_default: bool,
    /// 默认值
    pub default: String,
}

/// Cocos 管理器模板
#[derive(Template)]
#[template(path = "BuildManager.ts.dejavu", escape = "none")]
pub struct CocosManagerTemplate {
    /// 编译器版本
    compiler_version: &'static str,
    /// 管理器名称
    class_name: String,
    /// 实例名称
    instance_name: String,
    /// Cocos 代码生成配置
    config: CocosCodegen,
    /// 数据版本
    data_version: String,
    /// 编辑时间
    edit_time: String,
    /// 表列表
    tables: Vec<TableItem>,
}

/// 表项数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableItem {
    /// 私有名称
    pub private_name: String,
    /// 公共名称
    pub public_name: String,
    /// 类型
    pub typing: String,
}

// 使用dejavu模板生成代码
fn render_enumerate_template(config: &CocosCodegen, class_name: &str, items: &[CocosEnumerateItem]) -> XResult<String> {
    let enumerate_ids: Vec<EnumeratePair> = items.iter().map(|item| EnumeratePair {
        key: item.key.clone(),
        value: item.id.to_string(),
        document: vec![item.description.clone()],
    }).collect();
    let template = CocosEnumerateTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        class_name: class_name.to_string(),
        id_type: "number".to_string(),
        config: config.clone(),
        enumerate_ids,
        class_document: vec![],
    };
    template.render(&dejavu_types::values::Context::new()).map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
}

fn render_class_template(config: &CocosCodegen, class_name: &str, table_name: &str, fields: &[CocosField]) -> XResult<String> {
    let class_fields: Vec<ClassFieldTemplate> = fields.iter().map(|field| ClassFieldTemplate {
        document: vec![],
        name: field.name.clone(),
        typing: field.r#type.clone(),
        has_default: false,
        default: String::new(),
    }).collect();
    let template = CocosClassTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        class_name: class_name.to_string(),
        table_name: table_name.to_string(),
        id_type: "number".to_string(),
        config: config.clone(),
        key_name: "id".to_string(),
        class_document: vec![],
        class_fields,
    };
    template.render(&dejavu_types::values::Context::new()).map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
}

fn render_manager_template(config: &CocosCodegen, tables: &[CocosDataTableItem]) -> XResult<String> {
    let table_items: Vec<TableItem> = tables.iter().map(|table| TableItem {
        private_name: table.cache_name.clone(),
        public_name: table.get_method_name.clone(),
        typing: table.table_name.clone(),
    }).collect();
    let template = CocosManagerTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        class_name: config.manager_name.clone(),
        instance_name: config.instance_name.clone(),
        config: config.clone(),
        data_version: "1.0.0".to_string(),
        edit_time: chrono::Utc::now().to_rfc3339(),
        tables: table_items,
    };
    template.render(&dejavu_types::values::Context::new()).map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
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

/// 类型映射配置项
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeMapping {
    /// Rust 类型
    pub rust_type: String,
    /// TypeScript 类型
    pub ts_type: String,
}

/// Cocos 代码生成器配置
///
/// 用于配置 Cocos 平台的代码生成
#[derive(Clone, Debug, Serialize)]
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
    /// 特殊字段名配置
    pub special_fields: Option<Vec<String>>,
    /// 特殊类名配置
    pub special_classes: Option<Vec<String>>,
    /// 枚举后缀配置
    pub enum_suffixes: Option<Vec<String>>,
    /// 类型映射配置
    pub type_mappings: Option<Vec<TypeMapping>>,
    /// CSV 文件缓存
    #[serde(skip)]
    pub(crate) cache: CsvCache,
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

impl Default for CocosCodegen {
    fn default() -> Self {
        Self {
            storage: Default::default(),
            enable: false,
            project: String::new(),
            output: String::new(),
            namespace: String::new(),
            manager_name: "DataTableManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "dataTableManager".to_string(),
            table_data_path: "tables".to_string(),
            special_fields: Some(vec!["type".to_string(), "level".to_string(), "level_requirement".to_string(), "drop_items".to_string(), "skills".to_string(), "unlock_skills".to_string()]),
            special_classes: Some(vec!["Monster".to_string(), "Skill".to_string()]),
            enum_suffixes: Some(vec!["Type".to_string(), "Kind".to_string()]),
            type_mappings: Some(vec![
                TypeMapping { rust_type: "i32".to_string(), ts_type: "number".to_string() },
                TypeMapping { rust_type: "i64".to_string(), ts_type: "number".to_string() },
                TypeMapping { rust_type: "u32".to_string(), ts_type: "number".to_string() },
                TypeMapping { rust_type: "u64".to_string(), ts_type: "number".to_string() },
                TypeMapping { rust_type: "f32".to_string(), ts_type: "number".to_string() },
                TypeMapping { rust_type: "f64".to_string(), ts_type: "number".to_string() },
                TypeMapping { rust_type: "text".to_string(), ts_type: "string".to_string() },
                TypeMapping { rust_type: "string".to_string(), ts_type: "string".to_string() },
                TypeMapping { rust_type: "any".to_string(), ts_type: "string".to_string() },
            ]),
            cache: Default::default(),
        }
    }
}

/// CSV 文件缓存结构
#[derive(Debug, Clone, Default)]
pub struct CsvCache {
    /// 字段信息缓存
    pub fields_cache: std::collections::HashMap<PathBuf, Vec<CocosField>>,
    /// 枚举数据缓存
    pub enum_cache: std::collections::HashMap<PathBuf, Vec<(u32, String, String)>>,
    /// CSV 文件列表缓存（存储路径而非 DirEntry，因为 DirEntry 不实现 Clone）
    pub files_cache: Option<Vec<PathBuf>>,
}

/// Cocos 代码生成器
///
/// 负责生成 Cocos 平台的代码和数据文件
impl CocosCodegen {
    /// 创建新的 CocosCodegen 实例
    pub fn new() -> Self {
        Self::default()
    }
    
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
        tracing::info!("cocos_project_path: {:?}", path);
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
        tracing::debug!("cocos_typescript_path: file_name={}, path={:?}", file_name, path);
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
        tracing::debug!("cocos_json_path: file_name={}, path={:?}", file_name, path);
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

    /// 检查是否是枚举类型
    fn is_enum(&self, class_name: &str) -> bool {
        if let Some(suffixes) = &self.enum_suffixes {
            suffixes.iter().any(|suffix| class_name.ends_with(suffix))
        } else {
            class_name.ends_with("Type") || class_name.ends_with("Kind")
        }
    }
    
    /// 检查是否是特殊字段
    fn is_special_field(&self, field_name: &str) -> bool {
        if let Some(fields) = &self.special_fields {
            fields.contains(&field_name.to_string())
        } else {
            config::is_special_field(field_name)
        }
    }
    
    /// 检查是否是特殊类名
    fn is_special_class(&self, class_name: &str) -> bool {
        if let Some(classes) = &self.special_classes {
            classes.contains(&class_name.to_string())
        } else {
            config::is_special_class(class_name)
        }
    }
    
    /// 写入 TypeScript 代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    pub fn write_typescript(&mut self, ws: &WorkspaceManager) -> XResult<()> {
        tracing::info!("CocosCodegen::write_typescript called");
        
        let root = &ws.config.root;
        
        if let Some(s) = self.cocos_typescript_path(root, "DataTableManager")?.parent() {
            std::fs::create_dir_all(s)?;
        }
        
        let csv_files = self.get_csv_files(root)?;
        
        for path in &csv_files {
            let file_name = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let class_name = file_name.split('.').next().unwrap_or(&file_name);
            
            let is_enum = self.is_enum(class_name);
            
            if is_enum {
                self.process_enum_file(ws, path, class_name)?;
            } else {
                self.process_table_file(ws, path, class_name)?;
            }
        }
        
        self.write_data_table_manager(ws)?;
        
        Ok(())
    }
    
    /// 处理枚举文件
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `path` - 文件路径
    /// * `class_name` - 类名
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    fn process_enum_file(&mut self, ws: &WorkspaceManager, path: &PathBuf, class_name: &str) -> XResult<()> {
        let root = &ws.config.root;
        let ts_path = self.cocos_typescript_path(root, class_name)?;
        
        tracing::info!("processing_enum: class_name={}, output_path={:?}", class_name, ts_path);
        
        if let Some(parent) = ts_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let enum_data = self.read_enum_data(path)?;
        
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
        
        let content = render_enumerate_template(self, class_name, &items)?;
        
        let mut file = File::create(ts_path)?;
        file.write_all(content.as_bytes())?;
        
        tracing::info!("created_typescript_enum: class_name={}", class_name);
        Ok(())
    }
    
    /// 处理表格文件
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `path` - 文件路径
    /// * `class_name` - 类名
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    fn process_table_file(&mut self, ws: &WorkspaceManager, path: &PathBuf, class_name: &str) -> XResult<()> {
        let root = &ws.config.root;
        let table_class_name = format!("{}Table", class_name);
        let ts_path = self.cocos_typescript_path(root, &table_class_name)?;
        
        tracing::info!("processing_table: class_name={}, table_class_name={}, output_path={:?}", class_name, &table_class_name, ts_path);
        
        if let Some(parent) = ts_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let fields = self.read_csv_fields(path, class_name)?;
        
        let content = render_class_template(self, class_name, &table_class_name, &fields)?;
        
        let mut file = File::create(ts_path)?;
        file.write_all(content.as_bytes())?;
        
        tracing::info!("created_typescript_file: class_name={}", class_name);
        Ok(())
    }
    
    /// 获取 CSV 文件列表
    ///
    /// # 参数
    /// * `root` - 根目录路径
    ///
    /// # 返回值
    /// 返回 CSV 文件路径列表，成功时返回 Ok(Vec<PathBuf>)，失败时返回 XError。
    pub fn get_csv_files(&mut self, root: &Path) -> XResult<Vec<PathBuf>> {
        if let Some(cache) = &self.cache.files_cache {
            tracing::debug!("Using cached CSV files list");
            return Ok(cache.clone());
        }
        
        let csv_files: Vec<PathBuf> = std::fs::read_dir(root)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension().map(|ext| ext == "csv").unwrap_or(false)
            })
            .map(|entry| entry.path())
            .collect();
        
        self.cache.files_cache = Some(csv_files.clone());
        Ok(csv_files)
    }
    
    /// 读取 CSV 文件的字段信息（带缓存）
    ///
    /// # 参数
    /// * `csv_path` - CSV 文件路径
    /// * `class_name` - 类名
    ///
    /// # 返回值
    /// 返回字段信息列表，成功时返回 Ok(Vec<CocosField>)，失败时返回 XError。
    pub fn read_csv_fields(&mut self, csv_path: &Path, class_name: &str) -> XResult<Vec<CocosField>> {
        if let Some(cached) = self.cache.fields_cache.get(csv_path) {
            tracing::debug!("Using cached fields for {:?}", csv_path);
            return Ok(cached.clone());
        }
        
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
        
        self.cache.fields_cache.insert(csv_path.to_path_buf(), fields.clone());
        Ok(fields)
    }
    
    /// 读取枚举类型的 CSV 文件数据（带缓存）
    ///
    /// # 参数
    /// * `csv_path` - CSV 文件路径
    ///
    /// # 返回值
    /// 返回枚举数据列表，每个元素包含 id、name 和 description，成功时返回 Ok(Vec<(u32, String, String)>)，失败时返回 XError。
    pub fn read_enum_data(&mut self, csv_path: &Path) -> XResult<Vec<(u32, String, String)>> {
        if let Some(cached) = self.cache.enum_cache.get(csv_path) {
            tracing::debug!("Using cached enum data for {:?}", csv_path);
            return Ok(cached.clone());
        }
        
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
        
        self.cache.enum_cache.insert(csv_path.to_path_buf(), enum_data.clone());
        Ok(enum_data)
    }
    
    /// 获取类型映射
    fn get_type_mapping(&self, rust_type: &str) -> String {
        if let Some(mappings) = &self.type_mappings {
            for mapping in mappings {
                if mapping.rust_type == rust_type {
                    return mapping.ts_type.clone();
                }
            }
        }
        config::get_type_mapping(rust_type).to_string()
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
        
        // 检查是否是数组类型
        if trimmed_type == "string[]" {
            return "number[]".to_string();
        }
        
        // 检查是否是需要特殊处理的字段
        let special_array_fields = vec!["drop_items", "skills", "unlock_skills"];
        if (trimmed_type == "text" || trimmed_type == "string") && special_array_fields.contains(&field_name) {
            return "number[]".to_string();
        }
        
        // 检查是否是类型字段且是特殊类
        if trimmed_type == "text" || trimmed_type == "string" {
            if field_name == "type" && self.is_special_class(class_name) {
                return format!("{}Type", class_name).to_string();
            }
        }
        
        // 使用配置的类型映射
        self.get_type_mapping(trimmed_type)
    }
    
    /// 写入 DataTableManager.ts
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    pub fn write_data_table_manager(&mut self, ws: &WorkspaceManager) -> XResult<()> {
        let root = &ws.config.root;
        
        let csv_files = self.get_csv_files(root)?;
        
        let manager_path = self.cocos_typescript_path(root, "DataTableManager")?;
        
        tracing::info!("generating_data_table_manager: output_path={:?}", manager_path);
        
        if let Some(parent) = manager_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let table_data_path = if self.table_data_path.is_empty() {
            "tables/"
        } else if self.table_data_path.ends_with('/') || self.table_data_path.ends_with('\\') {
            &self.table_data_path
        } else {
            &format!("{}/", self.table_data_path)
        };
        
        let mut tables = Vec::new();
        
        for entry in &csv_files {
            let file_name = entry.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let class_name = file_name.split('.').next().unwrap_or(&file_name);
            
            let is_enum = self.is_enum(class_name);
            
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
        
        let content = render_manager_template(self, &tables)?;
        
        let mut file = File::create(manager_path)?;
        file.write_all(content.as_bytes())?;
        
        tracing::info!("created_data_table_manager");
        
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
            self.process_class_json(root, table)?;
        }
        
        // 处理列表
        for table in ws.lists() {
            self.process_list_json(root, table)?;
        }
        
        // 处理字典
        for table in ws.dicts() {
            self.process_dict_json(root, table)?;
        }

        Ok(())
    }
    
    /// 处理类表 JSON 生成
    fn process_class_json(&self, root: &Path, table: &XClassData) -> XResult<()> {
        let json_path = self.cocos_json_path(root, &table.name)?;
        
        tracing::info!("generating_json: table_name={}, output_path={:?}", &table.name, json_path);
        
        if let Some(parent) = json_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let json_data = self.convert_class_data_to_json(table)?;
        let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
        
        let mut file = std::fs::File::create(json_path)?;
        file.write_all(json_string.as_bytes())?;
        
        tracing::info!("created_json_file: table_name={}", &table.name);
        Ok(())
    }
    
    /// 处理列表 JSON 生成
    fn process_list_json(&self, root: &Path, table: &XListData) -> XResult<()> {
        let json_path = self.cocos_json_path(root, &table.name)?;
        
        tracing::info!("generating_json: table_name={}, output_path={:?}", &table.name, json_path);
        
        if let Some(parent) = json_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let json_data = self.convert_list_data_to_json(table)?;
        let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
        
        let mut file = std::fs::File::create(json_path)?;
        file.write_all(json_string.as_bytes())?;
        
        tracing::info!("created_json_file: table_name={}", &table.name);
        Ok(())
    }
    
    /// 处理字典 JSON 生成
    fn process_dict_json(&self, root: &Path, table: &XDictData) -> XResult<()> {
        let json_path = self.cocos_json_path(root, &table.name)?;
        
        tracing::info!("generating_json: table_name={}, output_path={:?}", &table.name, json_path);
        
        if let Some(parent) = json_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let json_data = self.convert_dict_data_to_json(table)?;
        let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
        
        let mut file = std::fs::File::create(json_path)?;
        file.write_all(json_string.as_bytes())?;
        
        tracing::info!("created_json_file: table_name={}", &table.name);
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
            XCellValue::Boolean(b) => self.convert_boolean_to_json(*b),
            XCellValue::Integer8(i) => self.convert_integer_to_json(*i),
            XCellValue::Integer16(i) => self.convert_integer_to_json(*i),
            XCellValue::Integer32(i) => self.convert_integer_to_json(*i),
            XCellValue::Integer64(i) => self.convert_integer_to_json(*i),
            XCellValue::Unsigned8(u) => self.convert_unsigned_to_json(*u),
            XCellValue::Unsigned16(u) => self.convert_unsigned_to_json(*u),
            XCellValue::Unsigned32(u) => self.convert_unsigned_to_json(*u),
            XCellValue::Unsigned64(u) => self.convert_unsigned_to_json(*u),
            XCellValue::Float32(f) => self.convert_float_to_json(*f as f64),
            XCellValue::Float64(f) => self.convert_float_to_json(*f),
            XCellValue::String(s) => self.convert_string_to_json(s),
            XCellValue::Vector2(v) => self.convert_vector2_to_json(v),
            XCellValue::Vector3(v) => self.convert_vector3_to_json(v),
            XCellValue::Vector4(v) => self.convert_vector4_to_json(v),
            XCellValue::Quaternion4(v) => self.convert_quaternion4_to_json(v),
            XCellValue::Color(c) => self.convert_color_to_json(c),
            XCellValue::Vector(v) => self.convert_vector_to_json(v),
            XCellValue::Enumerate(s) => self.convert_enumerate_to_json(s),
            _ => serde_json::Value::Null,
        }
    }
    
    /// 将布尔值转换为 JSON
    fn convert_boolean_to_json(&self, value: bool) -> serde_json::Value {
        serde_json::Value::Bool(value)
    }
    
    /// 将整数转换为 JSON
    fn convert_integer_to_json<T: Into<i64>>(&self, value: T) -> serde_json::Value {
        serde_json::Value::Number(serde_json::Number::from(value.into()))
    }
    
    /// 将无符号整数转换为 JSON
    fn convert_unsigned_to_json<T: Into<u64>>(&self, value: T) -> serde_json::Value {
        serde_json::Value::Number(serde_json::Number::from(value.into()))
    }
    
    /// 将浮点数转换为 JSON
    fn convert_float_to_json(&self, value: f64) -> serde_json::Value {
        serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap())
    }
    
    /// 将字符串转换为 JSON
    fn convert_string_to_json(&self, value: &str) -> serde_json::Value {
        serde_json::Value::String(value.to_string())
    }
    
    /// 将 Vector2 转换为 JSON
    fn convert_vector2_to_json(&self, value: &[f32; 2]) -> serde_json::Value {
        serde_json::Value::Array(vec![
            self.convert_float_to_json(value[0] as f64),
            self.convert_float_to_json(value[1] as f64)
        ])
    }
    
    /// 将 Vector3 转换为 JSON
    fn convert_vector3_to_json(&self, value: &[f32; 3]) -> serde_json::Value {
        serde_json::Value::Array(vec![
            self.convert_float_to_json(value[0] as f64),
            self.convert_float_to_json(value[1] as f64),
            self.convert_float_to_json(value[2] as f64)
        ])
    }
    
    /// 将 Vector4 转换为 JSON
    fn convert_vector4_to_json(&self, value: &[f32; 4]) -> serde_json::Value {
        serde_json::Value::Array(vec![
            self.convert_float_to_json(value[0] as f64),
            self.convert_float_to_json(value[1] as f64),
            self.convert_float_to_json(value[2] as f64),
            self.convert_float_to_json(value[3] as f64)
        ])
    }
    
    /// 将 Quaternion4 转换为 JSON
    fn convert_quaternion4_to_json(&self, value: &[f32; 4]) -> serde_json::Value {
        serde_json::Value::Array(vec![
            self.convert_float_to_json(value[0] as f64),
            self.convert_float_to_json(value[1] as f64),
            self.convert_float_to_json(value[2] as f64),
            self.convert_float_to_json(value[3] as f64)
        ])
    }
    
    /// 将 Color 转换为 JSON
    fn convert_color_to_json(&self, value: &xcell_core::for_3rd::Color) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::from_iter(vec![
            ("r".to_string(), self.convert_float_to_json(value.r as f64)),
            ("g".to_string(), self.convert_float_to_json(value.g as f64)),
            ("b".to_string(), self.convert_float_to_json(value.b as f64)),
            ("a".to_string(), self.convert_float_to_json(value.a as f64))
        ]))
    }
    
    /// 将 Vector 转换为 JSON
    fn convert_vector_to_json(&self, value: &Vec<XCellValue>) -> serde_json::Value {
        serde_json::Value::Array(
            value.iter().map(|item| self.convert_xcell_value_to_json(item)).collect()
        )
    }
    
    /// 将 Enumerate 转换为 JSON
    fn convert_enumerate_to_json(&self, value: &str) -> serde_json::Value {
        serde_json::Value::String(value.to_string())
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
        tracing::info!("CocosCodegen::generate called");
        tracing::info!("Output directory: {:?}", context.output_dir);
        
        if let Some(workspace) = &context.workspace {
            tracing::info!("Workspace root: {:?}", workspace.config.root);
            
            for generator in &workspace.config.generators {
                if let xcell_config::project::Generator::Cocos(cocos_config) = generator {
                    let mut cocos_codegen = CocosCodegen {
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
                        special_fields: Some(vec!["type".to_string(), "level".to_string(), "level_requirement".to_string(), "drop_items".to_string(), "skills".to_string(), "unlock_skills".to_string()]),
                        special_classes: Some(vec!["Monster".to_string(), "Skill".to_string()]),
                        enum_suffixes: Some(vec!["Type".to_string(), "Kind".to_string()]),
                        type_mappings: Some(vec![
                            TypeMapping { rust_type: "i32".to_string(), ts_type: "number".to_string() },
                            TypeMapping { rust_type: "i64".to_string(), ts_type: "number".to_string() },
                            TypeMapping { rust_type: "u32".to_string(), ts_type: "number".to_string() },
                            TypeMapping { rust_type: "u64".to_string(), ts_type: "number".to_string() },
                            TypeMapping { rust_type: "f32".to_string(), ts_type: "number".to_string() },
                            TypeMapping { rust_type: "f64".to_string(), ts_type: "number".to_string() },
                            TypeMapping { rust_type: "text".to_string(), ts_type: "string".to_string() },
                            TypeMapping { rust_type: "string".to_string(), ts_type: "string".to_string() },
                            TypeMapping { rust_type: "any".to_string(), ts_type: "string".to_string() },
                        ]),
                        cache: Default::default(),
                    };
                    
                    tracing::info!("Cocos codegen enable: {}", cocos_codegen.enable);
                    tracing::info!("Cocos project: {}", cocos_codegen.project);
                    tracing::info!("Cocos output: {}", cocos_codegen.output);
                    
                    tracing::info!("Calling write_typescript");
                    cocos_codegen.write_typescript(workspace)?;
                    tracing::info!("write_typescript completed");
                    tracing::info!("Calling write_json");
                    cocos_codegen.write_json(workspace)?;
                    tracing::info!("write_json completed");
                }
            }
        } else {
            tracing::warn!("No workspace manager in context");
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "cocos"
    }
}
