use xcell_analyzer::WorkspaceManager;
use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use xcell_types::{XError, XResult};
use url::Url;
use dejavu_macros::Template;

// 暂时禁用这些模块，因为它们依赖于不存在的方法
// mod class;
// mod dictionary;
// mod enumerate;
// mod manager;

#[derive(Template)]
#[template(path = "BuildCocosClass.ts", ext = "txt", escape = "none")]
pub struct CocosClassTemplate {
    /// Class name
    class_name: String,
    /// Table name
    table_name: String,
    /// Fields
    fields: Vec<CocosField>,
    /// Whether the class has a type field
    has_type_field: bool,
    /// Whether the class has a level field
    has_level_field: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CocosField {
    /// Field name
    name: String,
    /// Field type
    r#type: String,
}

/// Cocos 代码生成器配置
///
/// 用于配置 Cocos 平台的代码生成
#[derive(Clone, Debug, Default, Serialize)]
pub struct CocosCodegen {
    /// 是否启用 Cocos 代码生成
    pub enable: bool,
    /// Cocos 项目目录，推荐使用相对路径
    pub project: String,
    /// 输出目录
    pub output: String,
    /// 生成代码的命名空间
    pub namespace: String,
    /// 生成的管理器名称
    pub manager_name: String,
    /// 生成的表名后缀
    pub suffix_table: String,
    /// 生成的实例名称
    pub instance_name: String,
    /// JSON 配置
    pub json: CocosJsonConfig,
}

/// Cocos JSON 配置
///
/// 用于配置 JSON 数据生成
#[derive(Clone, Debug, Default, Serialize)]
pub struct CocosJsonConfig {
    /// 是否启用 JSON 生成
    pub enable: bool,
    /// 生成的 JSON 文件目录
    pub output: String,
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
        // 优先使用 cocos 子目录
        let cocos_subdir = root.join("cocos");
        if cocos_subdir.exists() {
            println!("Using cocos subdirectory: {:?}", cocos_subdir);
            return Ok(cocos_subdir);
        }
        
        // 如果 cocos 子目录不存在，使用配置中的 project 路径
        let project = PathBuf::from(&self.project);
        let project = match project.is_absolute() {
            true => project,
            false => root.join(project),
        };
        
        // 尝试规范化路径，如果失败则返回原始路径
        match project.canonicalize() {
            Ok(canonical_path) => {
                println!("Cocos project canonical path: {:?}", canonical_path);
                Ok(canonical_path)
            },
            Err(e) => {
                println!("Failed to canonicalize Cocos project path: {:?}, using original path: {:?}", e, project);
                Ok(project)
            }
        }
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
        let cocos_path = self.cocos_path(root)?;
        
        // 处理空的 output 字段
        let output_path = if self.output.is_empty() {
            // 默认输出到 assets/scripts/dataTable/generated 目录
            cocos_path.join("assets").join("scripts").join("dataTable").join("generated")
        } else {
            cocos_path.join(&self.output)
        };
        
        let path = output_path.join(file_name).with_extension("ts");
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
        let dir = self.cocos_path(root)?.join(&self.json.output);
        let path = dir.join(file_name).with_extension("json");
        Ok(path)
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
        format!("{}/{}.json", self.json.output, file_name)
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
            if self.json.enable {
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
        
        let cocos = &ws.config.cocos;
        let root = &ws.config.root;
        
        println!("Cocos enable: {}", cocos.loader.enable);
        println!("Cocos project: {:?}", cocos.loader.project);
        println!("Cocos output: {:?}", cocos.loader.output);
        println!("Cocos namespace: {:?}", cocos.loader.namespace);
        
        // 确保输出目录存在
        if let Some(s) = self.cocos_typescript_path(root, "test")?.parent() {
            std::fs::create_dir_all(s)?;
        }
        if cocos.storage.json.enable {
            if let Some(s) = self.cocos_json_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }

        use std::fs;
        
        // 读取 CSV 文件
        let csv_files = fs::read_dir(root)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "csv")
            })
            .collect::<Vec<_>>();
        
        println!("Found {} CSV files", csv_files.len());
        
        for entry in &csv_files {
            let entry_path = entry.path();
            let csv_path = entry_path.to_path_buf();
            let file_stem = csv_path.file_stem().unwrap();
            let file_name = file_stem.to_str().unwrap();
            let table_name = format!("{}Table", file_name);
            let ts_path = self.cocos_typescript_path(root, &table_name)?;
            
            println!("Processing CSV file: {} -> {}", csv_path.display(), ts_path.display());
            
            // 创建目录
            if let Some(parent) = ts_path.parent() {
                fs::create_dir_all(parent)?;
                println!("Created directory: {:?}", parent);
            }
            
            // 读取 CSV headers
            let mut rdr = csv::Reader::from_path(csv_path.clone()).map_err(|e| XError::runtime_error(format!("CSV reader error: {}", e)))?;
            let headers = rdr.headers().map_err(|e| XError::runtime_error(format!("CSV headers error: {}", e)))?;
            
            // 准备字段数据
            let mut fields = Vec::new();
            let mut has_type_field = false;
            let mut has_level_field = false;
            
            for header in headers.iter() {
                let field_name = header;
                // 简单类型推断
                let field_type = match field_name {
                    "id" | "level" | "attack" | "defense" | "health" | "damage" | "required_exp" => "string",
                    "name" | "type" | "description" | "drop_items" | "skills" | "unlock_skills" => "string",
                    _ => "string",
                };
                
                fields.push(CocosField {
                    name: field_name.to_string(),
                    r#type: field_type.to_string(),
                });
                
                if field_name == "type" {
                    has_type_field = true;
                } else if field_name == "level" {
                    has_level_field = true;
                }
            }
            
            // 直接生成 TypeScript 代码
            let mut code = String::new();
            code.push_str(&format!("// {}.ts\n", table_name));
            code.push_str("import { DataTableManager } from './DataTableManager';\n\n");
            code.push_str(&format!("export class {} {{\n", table_name));
            code.push_str("    private data: any[] = [];\n    private idMap: Map<string, any> = new Map();\n");
            
            if has_type_field {
                code.push_str("    private typeMap: Map<string, any[]> = new Map();\n");
            }
            if has_level_field {
                code.push_str("    private levelMap: Map<string, any[]> = new Map();\n");
            }
            
            code.push_str("\n");
            code.push_str("    constructor() {}\n\n");
            code.push_str("    load(data: any[]): void {\n");
            code.push_str("        this.data = data;\n");
            code.push_str("        this.idMap.clear();\n");
            
            if has_type_field {
                code.push_str("        this.typeMap.clear();\n");
            }
            if has_level_field {
                code.push_str("        this.levelMap.clear();\n");
            }
            
            code.push_str("\n");
            code.push_str("        for (const item of data) {\n");
            code.push_str("            this.idMap.set(item.id, item);\n");
            
            if has_type_field {
                code.push_str("            if (item.type) {\n");
                code.push_str("                if (!this.typeMap.has(item.type)) {\n");
                code.push_str("                    this.typeMap.set(item.type, []);\n");
                code.push_str("                }\n");
                code.push_str("                this.typeMap.get(item.type)?.push(item);\n");
                code.push_str("            }\n");
            }
            
            if has_level_field {
                code.push_str("            if (item.level) {\n");
                code.push_str("                if (!this.levelMap.has(item.level)) {\n");
                code.push_str("                    this.levelMap.set(item.level, []);\n");
                code.push_str("                }\n");
                code.push_str("                this.levelMap.get(item.level)?.push(item);\n");
                code.push_str("            }\n");
            }
            
            code.push_str("        }\n");
            code.push_str("    }\n\n");
            code.push_str("    getById(id: string): any {\n");
            code.push_str("        return this.idMap.get(id);\n");
            code.push_str("    }\n\n");
            
            if has_type_field {
                code.push_str("    getByType(type: string): any[] {\n");
                code.push_str("        return this.typeMap.get(type) || [];\n");
                code.push_str("    }\n\n");
            }
            
            if has_level_field {
                code.push_str("    getByLevel(level: string): any[] {\n");
                code.push_str("        return this.levelMap.get(level) || [];\n");
                code.push_str("    }\n\n");
            }
            
            code.push_str("    getAll(): any[] {\n");
            code.push_str("        return this.data;\n");
            code.push_str("    }\n");
            code.push_str("}\n");
            
            // 写入文件
            let mut file = File::create(ts_path)?;
            file.write_all(code.as_bytes())?;
            
            println!("Created TypeScript file for {} successfully", file_name);
        }
        
        // 生成 DataTableManager
        let manager_path = self.cocos_typescript_path(root, &self.manager_name)?;
        let mut manager_code = String::new();
        manager_code.push_str("// DataTableManager.ts\n");
        
        // 添加导入语句
        for entry in &csv_files {
            let file_name = entry.path().file_stem().unwrap().to_str().unwrap();
            let table_name = format!("{}Table", file_name);
            manager_code.push_str(&format!("import {{ {} }} from './{}';\n", table_name, table_name));
        }
        
        manager_code.push_str("\n");
        manager_code.push_str("export class DataTableManager {\n");
        manager_code.push_str("    private static instance: DataTableManager;\n");
        
        // 添加表实例
        for entry in &csv_files {
            let file_name = entry.path().file_stem().unwrap().to_str().unwrap();
            let table_name = format!("{}Table", file_name);
            manager_code.push_str(&format!("    private {}Table: {} | null = null;\n", file_name, table_name));
        }
        
        manager_code.push_str("\n");
        manager_code.push_str("    private constructor() {}\n\n");
        manager_code.push_str("    public static getInstance(): DataTableManager {\n");
        manager_code.push_str("        if (!DataTableManager.instance) {\n");
        manager_code.push_str("            DataTableManager.instance = new DataTableManager();\n");
        manager_code.push_str("        }\n");
        manager_code.push_str("        return DataTableManager.instance;\n");
        manager_code.push_str("    }\n\n");
        
        // 添加导入语句
        for entry in &csv_files {
            let file_name = entry.path().file_stem().unwrap().to_str().unwrap();
            let table_name = format!("{}Table", file_name);
            manager_code.push_str(&format!("    import{}(): Promise<void> {{\n", file_name));
            manager_code.push_str(&format!("        if (!this.{}Table) {{\n", file_name));
            manager_code.push_str(&format!("            this.{}Table = new {}Table();\n", file_name, table_name));
            manager_code.push_str(&format!("            const data = await fetch('{}/{}.json').then(res => res.json());\n", self.json.output, file_name));
            manager_code.push_str(&format!("            this.{}Table.load(data);\n", file_name));
            manager_code.push_str("        }\n");
            manager_code.push_str("        return Promise.resolve();\n");
            manager_code.push_str("    }\n\n");
        }
        
        // 添加获取表实例的方法
        for entry in &csv_files {
            let file_name = entry.path().file_stem().unwrap().to_str().unwrap();
            let table_name = format!("{}Table", file_name);
            manager_code.push_str(&format!("    get{}Table(): {} {{\n", file_name, table_name));
            manager_code.push_str(&format!("        if (!this.{}Table) {{\n", file_name));
            manager_code.push_str(&format!("            this.{}Table = new {}Table();\n", file_name, table_name));
            manager_code.push_str("        }\n");
            manager_code.push_str(&format!("        return this.{}Table;\n", file_name));
            manager_code.push_str("    }\n\n");
        }
        
        manager_code.push_str("    async loadAll(): Promise<void> {\n");
        manager_code.push_str("        const promises = [\n");
        
        for entry in &csv_files {
            let file_name = entry.path().file_stem().unwrap().to_str().unwrap();
            manager_code.push_str(&format!("            this.import{}(),\n", file_name));
        }
        
        manager_code.push_str("        ];\n");
        manager_code.push_str("        await Promise.all(promises);\n");
        manager_code.push_str("    }\n");
        manager_code.push_str("}\n");
        
        // 写入 DataTableManager 文件
        let mut manager_file = File::create(manager_path)?;
        manager_file.write_all(manager_code.as_bytes())?;
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
        if !self.enable || !self.json.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

        // 简化实现，只创建必要的目录结构
        Ok(())
    }

    // 暂时移除这些方法，因为它们依赖于不存在的字段和方法
    // /// 写入类表 JSON 数据
    // ///
    // /// # 参数
    // /// * `ws` - 工作区管理器
    // /// * `table` - 类数据表
    // ///
    // /// # 返回值
    // /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    // fn write_class_json(&self, ws: &WorkspaceManager, table: &XClassData) -> XResult<()> {
    //     use serde_json::json;

    //     let mut file = self.log_json(ws, &table.name)?;
    //     let mut items = vec![];

    //     for item in &table.items {
    //         let mut item_data = serde_json::Map::new();
    //         item_data.insert("id".to_string(), json!(item.id));
    //         item_data.insert("key".to_string(), json!(item.key));

    //         for field in &item.fields {
    //             item_data.insert(field.name.clone(), json!(field.value));
    //         }

    //         items.push(item_data);
    //     }

    //     let json_data = json!(items);
    //     file.write_all(serde_json::to_string_pretty(&json_data)
    //         .map_err(|e| XError::runtime_error(format!("JSON serialization error: {}", e)))?
    //         .as_bytes())?;
    //     Ok(())
    // }

    // /// 写入字典表 JSON 数据
    // ///
    // /// # 参数
    // /// * `ws` - 工作区管理器
    // /// * `table` - 字典数据表
    // ///
    // /// # 返回值
    // /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    // fn write_dict_json(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
    //     use serde_json::json;

    //     let mut file = self.log_json(ws, &table.name)?;
    //     let mut items = vec![];

    //     for item in &table.items {
    //         let mut item_data = serde_json::Map::new();
    //         item_data.insert("id".to_string(), json!(item.id));
    //         item_data.insert("key".to_string(), json!(item.key));
    //         item_data.insert("value".to_string(), json!(item.value));

    //         items.push(item_data);
    //     }

    //     let json_data = json!(items);
    //     file.write_all(serde_json::to_string_pretty(&json_data)
    //         .map_err(|e| XError::runtime_error(format!("JSON serialization error: {}", e)))?
    //         .as_bytes())?;
    //     Ok(())
    // }

    // /// 写入列表表 JSON 数据
    // ///
    // /// # 参数
    // /// * `ws` - 工作区管理器
    // /// * `table` - 列表数据表
    // ///
    // /// # 返回值
    // /// 返回操作结果，成功时返回 Ok(())，失败时返回 XError。
    // fn write_list_json(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
    //     use serde_json::json;

    //     let mut file = self.log_json(ws, &table.name)?;
    //     let mut items = vec![];

    //     for item in &table.items {
    //         let mut item_data = serde_json::Map::new();
    //         item_data.insert("id".to_string(), json!(item.id));
    //         item_data.insert("value".to_string(), json!(item.value));

    //         items.push(item_data);
    //     }

    //     let json_data = json!(items);
    //     file.write_all(serde_json::to_string_pretty(&json_data)
    //         .map_err(|e| XError::runtime_error(format!("JSON serialization error: {}", e)))?
    //         .as_bytes())?;
    //     Ok(())
    // }

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
        
        // 从上下文中获取工作区管理器
        if let Some(workspace) = &context.workspace {
            println!("Workspace root: {:?}", workspace.config.root);
            // 写入 TypeScript 代码
            println!("Calling write_typescript");
            self.write_typescript(workspace)?;
            println!("write_typescript completed");
            // 写入 JSON 数据
            println!("Calling write_json");
            self.write_json(workspace)?;
            println!("write_json completed");
        } else {
            println!("No workspace manager in context");
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "cocos"
    }
}
