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

/// Cocos 枚举模板数据
pub struct CocosEnumerateTemplate {
    /// 编译器版本
    compiler_version: &'static str,
    /// 类名
    class_name: String,
    /// ID 类型
    id_type: String,
    /// 命名空间
    namespace: String,
    /// 枚举 ID 列表
    enumerate_ids: Vec<EnumeratePair>,
    /// 类文档
    class_document: Vec<String>,
}

/// Cocos 类模板数据
pub struct CocosClassTemplate {
    /// 编译器版本
    pub compiler_version: &'static str,
    /// 类名
    pub class_name: String,
    /// 表名
    pub table_name: String,
    /// ID 类型
    pub id_type: String,
    /// 命名空间
    pub namespace: String,
    /// 键名
    pub key_name: String,
    /// 类文档
    pub class_document: Vec<String>,
    /// 类字段
    pub class_fields: Vec<ClassFieldTemplate>,
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

/// Cocos 管理器模板数据
pub struct CocosManagerTemplate {
    /// 编译器版本
    pub compiler_version: &'static str,
    /// 管理器名称
    pub class_name: String,
    /// 实例名称
    pub instance_name: String,
    /// 命名空间
    pub namespace: String,
    /// 管理器名称
    pub manager_name: String,
    /// 数据版本
    pub data_version: String,
    /// 编辑时间
    pub edit_time: String,
    /// 表列表
    pub tables: Vec<TableItem>,
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

// 使用 nargo template 动态渲染 dejavu 模板
fn render_enumerate_template(config: &CocosCodegen, class_name: &str, items: &[CocosEnumerateItem]) -> XResult<String> {
    let enumerate_ids: Vec<EnumeratePair> = items.iter().map(|item| EnumeratePair {
        key: item.key.clone(),
        value: item.id.to_string(),
        document: vec![item.description.clone()],
    }).collect();
    
    // 创建 NargoValue 上下文
    let mut context_data = std::collections::HashMap::new();
    context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
    context_data.insert("class_name".to_string(), NargoValue::String(class_name.to_string()));
    context_data.insert("id_type".to_string(), NargoValue::String("number".to_string()));
    context_data.insert("namespace".to_string(), NargoValue::String(config.namespace.clone()));
    
    // 处理 enumerate_ids
    let enumerate_ids_value: Vec<NargoValue> = enumerate_ids.iter().map(|pair| {
        let mut pair_data = std::collections::HashMap::new();
        pair_data.insert("key".to_string(), NargoValue::String(pair.key.clone()));
        pair_data.insert("value".to_string(), NargoValue::String(pair.value.clone()));
        pair_data.insert("document".to_string(), NargoValue::Array(
            pair.document.iter().map(|doc| NargoValue::String(doc.clone())).collect()
        ));
        NargoValue::Object(pair_data)
    }).collect();
    context_data.insert("enumerate_ids".to_string(), NargoValue::Array(enumerate_ids_value));
    context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));
    
    let context = NargoValue::Object(context_data);
    
    // 创建模板加载器
    let template_dir = config.template_dir.as_deref().map(Path::new);
    let loader = TemplateLoader::new(template_dir)?;
    
    // 使用模板加载器渲染模板
    loader.render_template(TemplateType::Enumerate.file_name(), &context)
}

fn render_class_template(config: &CocosCodegen, class_name: &str, table_name: &str, fields: &[CocosField]) -> XResult<String> {
    let class_fields: Vec<ClassFieldTemplate> = fields.iter().map(|field| ClassFieldTemplate {
        document: vec![],
        name: field.name.clone(),
        typing: field.r#type.clone(),
        has_default: false,
        default: String::new(),
    }).collect();
    
    // 创建 NargoValue 上下文
    let mut context_data = std::collections::HashMap::new();
    context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
    context_data.insert("class_name".to_string(), NargoValue::String(class_name.to_string()));
    context_data.insert("table_name".to_string(), NargoValue::String(table_name.to_string()));
    context_data.insert("id_type".to_string(), NargoValue::String("number".to_string()));
    context_data.insert("namespace".to_string(), NargoValue::String(config.namespace.clone()));
    context_data.insert("key_name".to_string(), NargoValue::String("id".to_string()));
    context_data.insert("class_document".to_string(), NargoValue::Array(vec![]));
    
    // 处理 class_fields
    let class_fields_value: Vec<NargoValue> = class_fields.iter().map(|field| {
        let mut field_data = std::collections::HashMap::new();
        field_data.insert("document".to_string(), NargoValue::Array(
            field.document.iter().map(|doc| NargoValue::String(doc.clone())).collect()
        ));
        field_data.insert("name".to_string(), NargoValue::String(field.name.clone()));
        field_data.insert("typing".to_string(), NargoValue::String(field.typing.clone()));
        field_data.insert("has_default".to_string(), NargoValue::Bool(field.has_default));
        field_data.insert("default".to_string(), NargoValue::String(field.default.clone()));
        NargoValue::Object(field_data)
    }).collect();
    context_data.insert("class_fields".to_string(), NargoValue::Array(class_fields_value));
    
    let context = NargoValue::Object(context_data);
    
    // 创建模板加载器
    let template_dir = config.template_dir.as_deref().map(Path::new);
    let loader = TemplateLoader::new(template_dir)?;
    
    // 使用模板加载器渲染模板
    loader.render_template(TemplateType::Class.file_name(), &context)
}

fn render_manager_template(config: &CocosCodegen, tables: &[CocosDataTableItem]) -> XResult<String> {
    let table_items: Vec<TableItem> = tables.iter().map(|table| TableItem {
        private_name: table.cache_name.clone(),
        public_name: table.get_method_name.clone(),
        typing: table.table_name.clone(),
    }).collect();
    
    // 创建 NargoValue 上下文
    let mut context_data = std::collections::HashMap::new();
    context_data.insert("compiler_version".to_string(), NargoValue::String(env!("CARGO_PKG_VERSION").to_string()));
    context_data.insert("class_name".to_string(), NargoValue::String(config.manager_name.clone()));
    context_data.insert("instance_name".to_string(), NargoValue::String(config.instance_name.clone()));
    context_data.insert("namespace".to_string(), NargoValue::String(config.namespace.clone()));
    context_data.insert("manager_name".to_string(), NargoValue::String(config.manager_name.clone()));
    context_data.insert("data_version".to_string(), NargoValue::String("1.0.0".to_string()));
    context_data.insert("edit_time".to_string(), NargoValue::String(chrono::Utc::now().to_rfc3339()));
    // 添加 table_data_path
    let table_data_path = if config.table_data_path.is_empty() {
        "tables/"
    } else if config.table_data_path.ends_with('/') || config.table_data_path.ends_with('\\') {
        &config.table_data_path
    } else {
        &format!("{}/", config.table_data_path)
    };
    context_data.insert("table_data_path".to_string(), NargoValue::String(table_data_path.to_string()));
    
    // 处理 tables
    let tables_value: Vec<NargoValue> = table_items.iter().map(|table| {
        let mut table_data = std::collections::HashMap::new();
        table_data.insert("private_name".to_string(), NargoValue::String(table.private_name.clone()));
        table_data.insert("public_name".to_string(), NargoValue::String(table.public_name.clone()));
        table_data.insert("typing".to_string(), NargoValue::String(table.typing.clone()));
        NargoValue::Object(table_data)
    }).collect();
    context_data.insert("tables".to_string(), NargoValue::Array(tables_value));
    
    let context = NargoValue::Object(context_data);
    
    // 创建模板加载器
    let template_dir = config.template_dir.as_deref().map(Path::new);
    let loader = TemplateLoader::new(template_dir)?;
    
    // 使用模板加载器渲染模板
    loader.render_template(TemplateType::Manager.file_name(), &context)
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
        
        // 处理枚举表
        for enum_table in ws.enumerates() {
            self.process_enum_table(ws, enum_table)?;
        }
        
        // 处理类表
        for class_table in ws.classes() {
            self.process_class_table(ws, class_table)?;
        }
        
        // 处理列表表
        for list_table in ws.lists() {
            self.process_list_table(ws, list_table)?;
        }
        
        // 处理字典表
        for dict_table in ws.dicts() {
            self.process_dict_table(ws, dict_table)?;
        }
        
        self.write_data_table_manager(ws)?;
        
        Ok(())
    }
    
    /// 处理枚举表
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `enum_table` - 枚举表数据
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    fn process_enum_table(&mut self, ws: &WorkspaceManager, enum_table: &xcell_analyzer::XEnumerateData) -> XResult<()> {
        let root = &ws.config.root;
        let class_name = &enum_table.name;
        let ts_path = self.cocos_typescript_path(root, class_name)?;
        
        tracing::info!("processing_enum: class_name={}, output_path={:?}", class_name, ts_path);
        
        if let Some(parent) = ts_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let items = enum_table.items.iter()
            .map(|item| {
                let key = item.key.to_uppercase().replace(" ", "_");
                CocosEnumerateItem {
                    key,
                    id: item.id, 
                    name: item.key.clone(),
                    description: item.value.clone(),
                }
            })
            .collect::<Vec<_>>();
        
        let content = render_enumerate_template(self, class_name, &items)?;
        
        let mut file = File::create(ts_path)?;
        file.write_all(content.as_bytes())?;
        
        tracing::info!("created_typescript_enum: class_name={}", class_name);
        Ok(())
    }
    
    /// 从表字段生成 CocosField 列表
    ///
    /// # 参数
    /// * `headers` - 表字段头部信息
    /// * `class_name` - 类名
    ///
    /// # 返回值
    /// 返回 CocosField 列表
    fn generate_fields_from_headers(&self, headers: &[xcell_analyzer::XHeader], class_name: &str) -> Vec<CocosField> {
        headers.iter()
            .map(|header| {
                let ts_type = self.map_csv_type_to_typescript(&header.field_type, &header.field_name, class_name);
                CocosField {
                    name: header.field_name.clone(),
                    r#type: ts_type,
                }
            })
            .collect()
    }
    
    /// 处理类表
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `class_table` - 类表数据
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    fn process_class_table(&mut self, ws: &WorkspaceManager, class_table: &XClassData) -> XResult<()> {
        let root = &ws.config.root;
        let class_name = &class_table.name;
        let table_class_name = format!("{}Table", class_name);
        let ts_path = self.cocos_typescript_path(root, &table_class_name)?;
        
        tracing::info!("processing_class_table: class_name={}, table_class_name={}, output_path={:?}", class_name, &table_class_name, ts_path);
        
        if let Some(parent) = ts_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        // 类表没有 headers，我们需要从 items 中提取字段信息
        let mut fields = Vec::new();
        for item in &class_table.items {
            let ts_type = self.map_csv_type_to_typescript(&item.r#type, &item.field, class_name);
            fields.push(CocosField {
                name: item.field.clone(),
                r#type: ts_type,
            });
        }
        
        let content = render_class_template(self, class_name, &table_class_name, &fields)?;
        
        let mut file = File::create(ts_path)?;
        file.write_all(content.as_bytes())?;
        
        tracing::info!("created_typescript_file: class_name={}", class_name);
        Ok(())
    }
    
    /// 处理列表表
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `list_table` - 列表表数据
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    fn process_list_table(&mut self, ws: &WorkspaceManager, list_table: &XListData) -> XResult<()> {
        let root = &ws.config.root;
        let class_name = &list_table.name;
        let table_class_name = format!("{}Table", class_name);
        let ts_path = self.cocos_typescript_path(root, &table_class_name)?;
        
        tracing::info!("processing_list_table: class_name={}, table_class_name={}, output_path={:?}", class_name, &table_class_name, ts_path);
        
        if let Some(parent) = ts_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let fields = self.generate_fields_from_headers(&list_table.headers, class_name);
        
        let content = render_class_template(self, class_name, &table_class_name, &fields)?;
        
        let mut file = File::create(ts_path)?;
        file.write_all(content.as_bytes())?;
        
        tracing::info!("created_typescript_file: class_name={}", class_name);
        Ok(())
    }
    
    /// 处理字典表
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `dict_table` - 字典表数据
    ///
    /// # 返回值
    /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    fn process_dict_table(&mut self, ws: &WorkspaceManager, dict_table: &XDictData) -> XResult<()> {
        let root = &ws.config.root;
        let class_name = &dict_table.name;
        let table_class_name = format!("{}Table", class_name);
        let ts_path = self.cocos_typescript_path(root, &table_class_name)?;
        
        tracing::info!("processing_dict_table: class_name={}, table_class_name={}, output_path={:?}", class_name, &table_class_name, ts_path);
        
        if let Some(parent) = ts_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let fields = self.generate_fields_from_headers(&dict_table.headers, class_name);
        
        let content = render_class_template(self, class_name, &table_class_name, &fields)?;
        
        let mut file = File::create(ts_path)?;
        file.write_all(content.as_bytes())?;
        
        tracing::info!("created_typescript_file: class_name={}", class_name);
        Ok(())
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
        
        // 处理类表
        for class_table in ws.classes() {
            let class_name = &class_table.name;
            if !self.is_enum(class_name) {
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
        
        // 处理列表表
        for list_table in ws.lists() {
            let class_name = &list_table.name;
            if !self.is_enum(class_name) {
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
        
        // 处理字典表
        for dict_table in ws.dicts() {
            let class_name = &dict_table.name;
            if !self.is_enum(class_name) {
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
