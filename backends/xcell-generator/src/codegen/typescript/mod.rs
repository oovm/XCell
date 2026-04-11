use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use url::Url;

use serde::{Serialize, Deserialize};
use serde_json;
use xcell_core::{XError, XResult, XCellValue, for_3rd::ToPrimitive};
use xcell_analyzer::{WorkspaceManager, XClassData, XListData, XDictData, XEnumerateData};

use crate::template::{TemplateLoader, TemplateType};
use chrono;

mod enumerate;
mod class;
mod dictionary;
mod manager;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeScriptCodegen {
    /// Whether to generate TypeScript code
    pub enable: bool,
    /// TypeScript 项目的工作目录, 建议使用相对路径
    pub project: String,
    /// loader 的输出目录，以 `project` 为根目录
    pub output: String,
    /// 生成的管理器的名称
    pub manager_name: String,
    /// 生成的表格名的后缀
    pub suffix_table: String,
    /// 生成的实例名称
    pub instance_name: String,
    /// 数据文件的输出目录，以 `project` 为根目录
    pub storage: String,
    /// 运行时加载类型 (json)
    pub storage_type: String,
    /// 使用 dejavu 模板生成代码的模板目录，空字符串表示使用默认模板
    pub loader_template: String,
    /// 模板目录路径
    pub template_dir: Option<String>,
}

impl Default for TypeScriptCodegen {
    fn default() -> Self {
        TypeScriptCodegen {
            enable: false,
            project: "..".to_string(),
            output: "typescript".to_string(),
            manager_name: "XCellManager".to_string(),
            suffix_table: "Table".to_string(),
            instance_name: "xcell".to_string(),
            storage: "".to_string(),
            storage_type: "json".to_string(),
            loader_template: "".to_string(),
            template_dir: None,
        }
    }
}

impl TypeScriptCodegen {
    /// TypeScript 项目文件夹
    pub fn project_path(&self, root: &Path) -> XResult<PathBuf> {
        let project = PathBuf::from(&self.project);
        let project = match project.is_absolute() {
            true => project,
            false => root.join(project),
        };
        Ok(project.canonicalize()?)
    }

    /// TypeScript 输出目录
    pub fn ts_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.project_path(root)?.join(&self.output);
        let path = dir.join(file_name).with_extension("ts");
        Ok(path)
    }

    /// Manager path
    pub fn ts_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.ts_path(root, &self.manager_name)
    }

    /// TypeScript 相对路径
    pub fn ts_relative(&self, file_name: &str) -> String {
        format!("{}/{}.ts", self.output, file_name)
    }

    /// 数据文件路径
    pub fn data_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let storage_path = if self.storage.is_empty() {
            &self.output
        } else {
            &self.storage
        };
        let dir = self.project_path(root)?.join(storage_path);
        let path = dir.join(file_name).with_extension("json");
        Ok(path)
    }

    /// Ensure output directories exist
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.ts_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
            if let Some(s) = self.data_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }

    /// Write TypeScript code
    pub fn write_typescript(&self, ws: &WorkspaceManager) -> XResult<()> {
        if !self.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

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
    pub fn write_json(&self, ws: &WorkspaceManager) -> XResult<()> {
        if !self.enable || self.storage_type != "json" {
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
        let json_path = self.data_path(root, &table.name)?;
        
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
        let json_path = self.data_path(root, &table.name)?;
        
        tracing::info!("generating_json: table_name={}, output_path={:?}", &table.name, json_path);
        
        if let Some(parent) = json_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let mut json_data = serde_json::Map::new();
        
        for (key, line) in &table.mapping {
            let mut record = serde_json::Map::new();
            record.insert("id".to_string(), serde_json::json!(line.id.to_u64().unwrap_or(0)));
            record.insert("key".to_string(), serde_json::json!(line.key));
            
            // 处理数据字段
            for (i, header) in table.headers.iter().enumerate() {
                if i < line.data.len() {
                    record.insert(header.field_name.clone(), serde_json::to_value(&line.data[i]).unwrap());
                }
            }
            
            json_data.insert(key.to_string(), serde_json::Value::Object(record));
        }
        
        let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
        
        let mut file = std::fs::File::create(json_path)?;
        file.write_all(json_string.as_bytes())?;
        
        tracing::info!("created_json_file: table_name={}", &table.name);
        Ok(())
    }

    /// 处理字典 JSON 生成
    fn process_dict_json(&self, root: &Path, table: &XDictData) -> XResult<()> {
        let json_path = self.data_path(root, &table.name)?;
        
        tracing::info!("generating_json: table_name={}, output_path={:?}", &table.name, json_path);
        
        if let Some(parent) = json_path.parent() {
            std::fs::create_dir_all(parent)?;
            tracing::debug!("created_directory: path={:?}", parent);
        }
        
        let mut json_data = serde_json::Map::new();
        
        for (key, line) in &table.mapping {
            let mut record = serde_json::Map::new();
            record.insert("id".to_string(), serde_json::json!(line.id.to_u64().unwrap_or(0)));
            record.insert("key".to_string(), serde_json::json!(line.key));
            
            // 处理数据字段
            for (i, header) in table.headers.iter().enumerate() {
                if i < line.data.len() {
                    record.insert(header.field_name.clone(), serde_json::to_value(&line.data[i]).unwrap());
                }
            }
            
            json_data.insert(key.clone(), serde_json::Value::Object(record));
        }
        
        let json_string = serde_json::to_string_pretty(&json_data).map_err(|e| XError::runtime_error(format!("JSON serialize error: {}", e)))?;
        
        let mut file = std::fs::File::create(json_path)?;
        file.write_all(json_string.as_bytes())?;
        
        tracing::info!("created_json_file: table_name={}", &table.name);
        Ok(())
    }

    /// 渲染枚举模板
    fn render_enumerate_template(&self, class_name: &str, items: &[(String, u32, String, String)]) -> XResult<String> {
        let enumerate_ids = items.iter().map(|(key, id, name, description)| {
            (key.clone(), id.to_string(), description.clone())
        }).collect::<Vec<_>>();
        
        // 创建 serde_json::Value 上下文
        let mut context_data = serde_json::Map::new();
        context_data.insert("compiler_version".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), serde_json::Value::String(class_name.to_string()));
        context_data.insert("id_type".to_string(), serde_json::Value::String("number".to_string()));
        
        // 处理 enumerate_ids
        let enumerate_ids_value: Vec<serde_json::Value> = enumerate_ids.iter().map(|(key, value, document)| {
            let mut pair_data = serde_json::Map::new();
            pair_data.insert("key".to_string(), serde_json::Value::String(key.clone()));
            pair_data.insert("value".to_string(), serde_json::Value::String(value.clone()));
            pair_data.insert("document".to_string(), serde_json::Value::Array(
                vec![serde_json::Value::String(document.clone())]
            ));
            serde_json::Value::Object(pair_data)
        }).collect();
        context_data.insert("enumerate_ids".to_string(), serde_json::Value::Array(enumerate_ids_value));
        context_data.insert("class_document".to_string(), serde_json::Value::Array(vec![]));
        
        let context = serde_json::Value::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        loader.render_template(TemplateType::Enumerate.file_name(), &context)
    }

    /// 渲染类模板
    fn render_class_template(&self, class_name: &str, table_name: &str, fields: &[(String, String)]) -> XResult<String> {
        // 创建 serde_json::Value 上下文
        let mut context_data = serde_json::Map::new();
        context_data.insert("compiler_version".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), serde_json::Value::String(class_name.to_string()));
        context_data.insert("table_name".to_string(), serde_json::Value::String(table_name.to_string()));
        context_data.insert("id_type".to_string(), serde_json::Value::String("number".to_string()));
        context_data.insert("key_name".to_string(), serde_json::Value::String("id".to_string()));
        context_data.insert("class_document".to_string(), serde_json::Value::Array(vec![]));
        
        // 处理 class_fields
        let class_fields_value: Vec<serde_json::Value> = fields.iter().map(|(name, typing)| {
            let mut field_data = serde_json::Map::new();
            field_data.insert("document".to_string(), serde_json::Value::Array(
                vec![serde_json::Value::String(String::new())]
            ));
            field_data.insert("name".to_string(), serde_json::Value::String(name.clone()));
            field_data.insert("typing".to_string(), serde_json::Value::String(typing.clone()));
            field_data.insert("has_default".to_string(), serde_json::Value::Bool(false));
            field_data.insert("default".to_string(), serde_json::Value::String(String::new()));
            serde_json::Value::Object(field_data)
        }).collect();
        context_data.insert("class_fields".to_string(), serde_json::Value::Array(class_fields_value));
        
        let context = serde_json::Value::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        loader.render_template(TemplateType::Class.file_name(), &context)
    }

    /// 渲染管理器模板
    fn render_manager_template(&self, tables: &[(String, String, String, String)]) -> XResult<String> {
        let table_items = tables.iter().map(|(class_name, table_name, cache_name, get_method_name)| {
            (cache_name.clone(), get_method_name.clone(), table_name.clone())
        }).collect::<Vec<_>>();
        
        // 创建 serde_json::Value 上下文
        let mut context_data = serde_json::Map::new();
        context_data.insert("compiler_version".to_string(), serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()));
        context_data.insert("class_name".to_string(), serde_json::Value::String(self.manager_name.clone()));
        context_data.insert("instance_name".to_string(), serde_json::Value::String(self.instance_name.clone()));
        context_data.insert("manager_name".to_string(), serde_json::Value::String(self.manager_name.clone()));
        context_data.insert("data_version".to_string(), serde_json::Value::String("1.0.0".to_string()));
        context_data.insert("edit_time".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()));
        
        // 添加 table_data_path
        let table_data_path = if self.storage.is_empty() {
            format!("{}/", self.output)
        } else if self.storage.ends_with('/') || self.storage.ends_with('\\') {
            self.storage.clone()
        } else {
            format!("{}/", self.storage)
        };
        context_data.insert("table_data_path".to_string(), serde_json::Value::String(table_data_path.to_string()));
        
        // 处理 tables
        let tables_value: Vec<serde_json::Value> = table_items.iter().map(|(private_name, public_name, typing)| {
            let mut table_data = serde_json::Map::new();
            table_data.insert("private_name".to_string(), serde_json::Value::String(private_name.clone()));
            table_data.insert("public_name".to_string(), serde_json::Value::String(public_name.clone()));
            table_data.insert("typing".to_string(), serde_json::Value::String(typing.clone()));
            serde_json::Value::Object(table_data)
        }).collect();
        context_data.insert("tables".to_string(), serde_json::Value::Array(tables_value));
        
        let context = serde_json::Value::Object(context_data);
        
        // 创建模板加载器
        let template_dir = self.template_dir.as_deref().map(Path::new);
        let loader = TemplateLoader::new(template_dir)?;
        
        // 使用模板加载器渲染模板
        loader.render_template(TemplateType::Manager.file_name(), &context)
    }

    /// 将 XCellValue 类型映射为 TypeScript 类型
    fn map_xcell_type_to_typescript(&self, value: &XCellValue) -> String {
        match value {
            XCellValue::Boolean(_) => "boolean".to_string(),
            XCellValue::Integer8(_) | XCellValue::Integer16(_) | XCellValue::Integer32(_) | XCellValue::Integer64(_) => "number".to_string(),
            XCellValue::Unsigned8(_) | XCellValue::Unsigned16(_) | XCellValue::Unsigned32(_) | XCellValue::Unsigned64(_) => "number".to_string(),
            XCellValue::Float32(_) | XCellValue::Float64(_) => "number".to_string(),
            XCellValue::String(_) => "string".to_string(),
            XCellValue::Vector2(_) | XCellValue::Vector3(_) | XCellValue::Vector4(_) => "number[]".to_string(),
            XCellValue::Quaternion4(_) => "number[]".to_string(),
            XCellValue::Color(_) => "{ r: number, g: number, b: number, a: number }".to_string(),
            XCellValue::Vector(_) => "any[]".to_string(),
            XCellValue::Enumerate(_) => "string".to_string(),
            _ => "any".to_string(),
        }
    }

    /// Log TypeScript file creation
    fn log_typescript(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.ts_path(&ws.config.root, name)?;
        tracing::info!("写入 TypeScript: {}\n{}", self.ts_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

impl super::Codegen for TypeScriptCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        if let Some(workspace) = &context.workspace {
            let mut ts_codegen = TypeScriptCodegen {
                enable: true,
                project: context.options.get("project").cloned().unwrap_or("..".to_string()),
                output: context.options.get("output").cloned().unwrap_or("typescript".to_string()),
                manager_name: context.options.get("manager_name").cloned().unwrap_or("XCellManager".to_string()),
                suffix_table: context.options.get("suffix_table").cloned().unwrap_or("Table".to_string()),
                instance_name: context.options.get("instance_name").cloned().unwrap_or("xcell".to_string()),
                storage: context.options.get("storage").cloned().unwrap_or("".to_string()),
                storage_type: context.options.get("storage_type").cloned().unwrap_or("json".to_string()),
                loader_template: context.options.get("loader_template").cloned().unwrap_or("".to_string()),
                template_dir: context.options.get("loader_template").cloned(),
            };
            ts_codegen.write_typescript(workspace)?;
            ts_codegen.write_json(workspace)?;
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "typescript"
    }
}
