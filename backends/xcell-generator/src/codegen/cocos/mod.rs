use xcell_analyzer::WorkspaceManager;
use serde::{Serialize, Deserialize};
use serde_json;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use xcell_core::{XError, XResult, XCellValue, for_3rd::ToPrimitive};
use xcell_analyzer::{XClassData, XListData, XDictData};
use url::Url;
use nargo_template::{DejaVuAdapter, UnifiedTemplateEngine};
use nargo_types::NargoValue;
use crate::template::{TemplateLoader, TemplateType};

mod typing;
mod enumerate;
mod class;
mod dictionary;
mod manager;



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
    /// 模板目录路径
    pub template_dir: Option<String>,
    /// 特殊字段名配置
    pub special_fields: Option<Vec<String>>,
    /// 特殊类名配置
    pub special_classes: Option<Vec<String>>,
    /// 枚举后缀配置
    pub enum_suffixes: Option<Vec<String>>,
    /// 类型映射配置
    pub type_mappings: Option<Vec<TypeMapping>>,
    /// 使用 dejavu 模板生成代码的模板目录，空字符串表示使用默认模板
    pub loader_template: String,

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
            template_dir: None,
            special_fields: None,
            special_classes: None,
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
            loader_template: "".to_string(),
        }
    }
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
            loader_template: self.loader_template.clone(),
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
            typing::is_special_field(field_name)
        }
    }
    
    /// 检查是否是特殊类名
    fn is_special_class(&self, class_name: &str) -> bool {
        if let Some(classes) = &self.special_classes {
            classes.contains(&class_name.to_string())
        } else {
            typing::is_special_class(class_name)
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
        
        // 处理枚举表
        for enum_table in ws.enumerates() {
            self.write_enumerate(ws, enum_table)?;
        }
        
        // 处理类表
        for class_table in ws.classes() {
            self.write_class(ws, class_table)?;
        }
        
        // 处理列表表
        for list_table in ws.lists() {
            self.write_list(ws, list_table)?;
        }
        
        // 处理字典表
        for dict_table in ws.dicts() {
            self.write_dict(ws, dict_table)?;
        }
        
        self.write_manager(ws)?;
        
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
        
        let json_data = table.items.iter().map(|item| {
            serde_json::json!({
                "field": item.field,
                "default": item.default
            })
        }).collect::<Vec<_>>();
        
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
        
        let json_data = table.mapping.values().map(|line| {
            let mut record = serde_json::Map::new();
            record.insert("id".to_string(), serde_json::json!(line.id.to_u64().unwrap_or(0)));
            record.insert("key".to_string(), serde_json::json!(line.key));
            
            // 处理数据字段
            for (i, header) in table.headers.iter().enumerate() {
                if i < line.data.len() {
                    record.insert(header.field_name.clone(), serde_json::to_value(&line.data[i]).unwrap());
                }
            }
            
            serde_json::Value::Object(record)
        }).collect::<Vec<_>>();
        
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
        
        let json_data = table.mapping.values().map(|line| {
            let mut record = serde_json::Map::new();
            record.insert("id".to_string(), serde_json::json!(line.id.to_u64().unwrap_or(0)));
            record.insert("key".to_string(), serde_json::json!(line.key));
            
            // 处理数据字段
            for (i, header) in table.headers.iter().enumerate() {
                if i < line.data.len() {
                    record.insert(header.field_name.clone(), serde_json::to_value(&line.data[i]).unwrap());
                }
            }
            
            serde_json::Value::Object(record)
        }).collect::<Vec<_>>();
        
        let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
        
        let mut file = std::fs::File::create(json_path)?;
        file.write_all(json_string.as_bytes())?;
        
        tracing::info!("created_json_file: table_name={}", &table.name);
        Ok(())
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
                        template_dir: context.options.get("loader_template").cloned(),
                        special_fields: None,
                        special_classes: None,
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
                        loader_template: cocos_config.loader_template.clone(),
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
