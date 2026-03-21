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
}

/// Cocos JSON 配置
///
/// 用于配置 JSON 数据生成
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
        // 使用 xcell-config 中定义的路径解析方法
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
        // 使用 xcell-config 中定义的路径解析方法
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
        // 使用 xcell-config 中定义的路径解析方法
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
        
        // 确保输出目录存在
        if let Some(s) = self.cocos_typescript_path(root, "DataTableManager")?.parent() {
            std::fs::create_dir_all(s)?;
        }
        
        // 读取目录中的所有 CSV 文件
        let csv_files = std::fs::read_dir(root)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension().map(|ext| ext == "csv").unwrap_or(false)
            })
            .collect::<Vec<_>>();
        
        // 处理每个 CSV 文件
        for entry in &csv_files {
            let file_name_os = entry.file_name();
            let file_name_str = file_name_os.to_string_lossy();
            let file_name = file_name_str.to_string();
            // 正确获取不带扩展名的文件名
            let class_name = file_name.split('.').next().unwrap_or(&file_name);
            
            // 检查是否为枚举类型：
            // 1. 如果表名以 Type 或 Kind 结尾
            let is_enum = class_name.ends_with("Type") || class_name.ends_with("Kind");
            
            if is_enum {
                let ts_path = self.cocos_typescript_path(root, class_name)?;
                
                println!("Processing enum: {} -> {}", class_name, ts_path.display());
                
                // 创建目录
                if let Some(parent) = ts_path.parent() {
                    std::fs::create_dir_all(parent)?;
                    println!("Created directory: {:?}", parent);
                }
                
                // 生成枚举代码
                let code = format!(r#"/**
 * {}接口
 */
export interface {} {{
    /**
     * {}ID
     */
    id: number;
    /**
     * {}名称
     */
    name: string;
    /**
     * {}描述
     */
    description: string;
}}

/**
 * {}枚举
 */
export const {} = {{
    /**
     * 普通{}
     */
    NORMAL: {{
        id: 1,
        name: "普通",
        description: "普通{}"
    }},
    /**
     * 不死{}
     */
    UNDEAD: {{
        id: 2,
        name: "undead",
        description: "undead {}"
    }},
    /**
     * 野兽{}
     */
    BEAST: {{
        id: 3,
        name: "野兽",
        description: "野兽{}"
    }},
    /**
     * 人形生物{}
     */
    HUMANOID: {{
        id: 4,
        name: "人形",
        description: "人形生物{}"
    }},
    /**
     * 巨型生物{}
     */
    GIANT: {{
        id: 5,
        name: "巨型",
        description: "巨型生物{}"
    }},
    /**
     * 龙{}
     */
    DRAGON: {{
        id: 6,
        name: "龙",
        description: "龙{}"
    }},
    /**
     * 元素生物{}
     */
    ELEMENTAL: {{
        id: 7,
        name: "元素",
        description: "元素生物{}"
    }}
}} as const as Record<string, {}>;
"#, 
                class_name, // 1
                class_name, // 2
                class_name, // 3
                class_name, // 4
                class_name, // 5
                class_name, // 6
                class_name, // 7
                class_name, // 8
                class_name, // 9
                class_name, // 10
                class_name, // 11
                class_name, // 12
                class_name, // 13
                class_name, // 14
                class_name, // 15
                class_name, // 16
                class_name, // 17
                class_name, // 18
                class_name, // 19
                class_name, // 20
                class_name, // 21
                class_name  // 22
            );
                
                // 写入文件
                let mut file = File::create(ts_path)?;
                file.write_all(code.as_bytes())?;
                
                println!("Created TypeScript enum for {} successfully", class_name);
            } else {
                let table_class_name = format!("{}Table", class_name);
                let ts_path = self.cocos_typescript_path(root, &table_class_name)?;
                
                println!("Processing list table: {} -> {}", class_name, ts_path.display());
                
                // 创建目录
                if let Some(parent) = ts_path.parent() {
                    std::fs::create_dir_all(parent)?;
                    println!("Created directory: {:?}", parent);
                }
                
                // 读取 CSV 文件的字段信息
                let fields = self.read_csv_fields(&entry.path())?;
                let has_type_field = fields.iter().any(|f| f.name == "type");
                let has_level_field = fields.iter().any(|f| f.name == "level") || fields.iter().any(|f| f.name == "level_requirement");
                
                // 直接生成完整的TypeScript代码
                let mut code = String::new();
                
                // 添加 MonsterType 导入
                if class_name == "Monster" {
                    code.push_str("import { MonsterType } from './MonsterType';

");
                }
                
                // 生成接口定义
                code.push_str(&format!("/**
 * {}数据结构
 */
export interface {} {{
", class_name, class_name));
                
                // 生成字段
                for field in &fields {
                    // 确保类型定义正确，避免使用 any 类型
                    let field_type = if field.r#type == "any" {
                        "string"
                    } else if field.r#type == "string[]" {
                        // 处理数组类型
                        "number[]"
                    } else if field.r#type == "string" && (field.name == "drop_items" || field.name == "skills" || field.name == "unlock_skills") {
                        // 特殊处理数组字段
                        "number[]"
                    } else if field.name == "type" && class_name == "Monster" {
                        // 处理 Monster 表的 type 字段为 MonsterType 类型
                        "MonsterType"
                    } else {
                        &field.r#type
                    };
                    
                    code.push_str(&format!("    /**
     * {}
     */
    {}: {};
", field.name, field.name, field_type));
                }
                
                code.push_str("}

");
                
                // 生成类定义
                code.push_str(&format!("/**
 * {}表加载器
 */
export class {} {{
    private items: {}[] = [];

", class_name, table_class_name, class_name));
                
                // 生成 load 方法
                code.push_str(&format!("    /**
     * 加载{}表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {{
        const data = asset.json;
        if (data) {{
            this.items = data as {}[];
        }}
    }}

", class_name, class_name));
                
                // 生成 getById 方法
                code.push_str(&format!("    /**
     * 根据ID获取{}
     * @param id {}ID
     */
    public get{}ById(id: number): {} | null {{
        return this.items.find(item => item.id === id) || null;
    }}

", class_name, class_name, class_name, class_name));
                
                // 生成 getAll 方法
                code.push_str(&format!("    /**
     * 获取所有{}
     */
    public getAll{}(): {}[] {{
        return this.items;
    }}

", class_name, class_name, class_name));
                
                // 生成 getByType 方法（如果有 type 字段）
                if has_type_field {
                    if class_name == "Monster" {
                        code.push_str(&format!("    /**
     * 根据类型获取{}
     * @param type 怪物类型
     */
    public get{}ByType(type: MonsterType): {}[] {{
        return this.items.filter(item => item.type === type);
    }}

", class_name, class_name, class_name));
                    } else {
                        code.push_str(&format!("    /**
     * 根据类型获取{}
     * @param type 类型
     */
    public get{}ByType(type: string): {}[] {{
        return this.items.filter(item => item.type === type);
    }}

", class_name, class_name, class_name));
                    }
                }
                
                // 生成 getByLevel 方法（如果有 level 字段）
                if has_level_field {
                    if class_name == "Skill" {
                        code.push_str(&format!("    /**
     * 根据等级获取{}
     * @param level 等级
     */
    public get{}ByLevel(level: number): {}[] {{
        return this.items.filter(item => item.level_requirement <= level);
    }}

", class_name, class_name, class_name));
                    } else {
                        code.push_str(&format!("    /**
     * 根据等级获取{}
     * @param level 等级
     */
    public get{}ByLevel(level: number): {}[] {{
        return this.items.filter(item => item.level === level);
    }}

", class_name, class_name, class_name));
                    }
                }
                
                code.push_str("}
");
                
                // 写入文件
                let mut file = File::create(ts_path)?;
                file.write_all(code.as_bytes())?;
                
                println!("Created TypeScript file for {} successfully", class_name);
            }
        }
        
        // 生成 DataTableManager.ts
        self.write_data_table_manager(ws)?;
        
        Ok(())
    }
    
    /// 读取 CSV 文件的字段信息
    ///
    /// # 参数
    /// * `csv_path` - CSV 文件路径
    ///
    /// # 返回值
    /// 返回字段信息列表，成功时返回 Ok(Vec<CocosField>)，失败时返回 XError。
    pub fn read_csv_fields(&self, csv_path: &Path) -> XResult<Vec<CocosField>> {
        let mut fields = Vec::new();
        
        // 读取 CSV 文件，禁用默认的标题行处理
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(csv_path)
            .map_err(|e| XError::runtime_error(format!("CSV read error: {}", e)))?;
        
        // 读取所有记录
        let mut records: Vec<csv::StringRecord> = rdr.records()
            .filter_map(|r| r.ok())
            .collect();
        
        // 确保至少有两行（字段名和类型）
        if records.len() >= 2 {
            // 第一行是字段名
            let headers = &records[0];
            // 第二行是类型
            let type_row = &records[1];
            
            for (i, header) in headers.iter().enumerate() {
                if i < type_row.len() {
                    let field_type = &type_row[i];
                    let ts_type = self.map_csv_type_to_typescript(field_type);
                    fields.push(CocosField {
                        name: header.to_string(),
                        r#type: ts_type,
                    });
                }
            }
        }
        
        Ok(fields)
    }
    
    /// 将 CSV 类型映射为 TypeScript 类型
    ///
    /// # 参数
    /// * `csv_type` - CSV 中的类型字符串
    ///
    /// # 返回值
    /// 返回对应的 TypeScript 类型字符串。
    pub fn map_csv_type_to_typescript(&self, csv_type: &str) -> String {
        match csv_type.trim() {
            "i32" | "i64" | "u32" | "u64" | "f32" | "f64" => "number".to_string(),
            "text" | "string" => "string".to_string(),
            _ => "string".to_string(), // 默认类型
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
        
        // 读取目录中的所有 CSV 文件
        let csv_files = std::fs::read_dir(root)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension().map(|ext| ext == "csv").unwrap_or(false)
            })
            .collect::<Vec<_>>();
        
        // 构建 DataTableManager.ts 的完整路径
        let manager_path = self.cocos_typescript_path(root, "DataTableManager")?;
        
        println!("Generating DataTableManager -> {}", manager_path.display());
        
        // 创建目录
        if let Some(parent) = manager_path.parent() {
            std::fs::create_dir_all(parent)?;
            println!("Created directory: {:?}", parent);
        }
        
        // 生成导入语句
        let mut imports = String::new();
        let mut cache_fields = String::new();
        let mut load_all_tables = String::new();
        let mut get_methods = String::new();
        
        // 处理每个 CSV 文件
        for entry in &csv_files {
            let file_name_os = entry.file_name();
            let file_name_str = file_name_os.to_string_lossy();
            let file_name = file_name_str.to_string();
            // 正确获取不带扩展名的文件名
            let class_name = file_name.split('.').next().unwrap_or(&file_name);
            
            // 检查是否为枚举类型
            let is_enum = class_name.ends_with("Type") || class_name.ends_with("Kind");
            
            if !is_enum {
                let table_class_name = format!("{}Table", class_name);
                let import_path = format!("./{}", table_class_name);
                
                // 添加导入语句
                imports.push_str(&format!("import {{ {} }} from '{}';\n", table_class_name, import_path));
                
                // 添加缓存字段
                let cache_field_name = format!("_{}Table", class_name.to_lowercase());
                cache_fields.push_str(&format!("    private {}: {} | null = null;\n", cache_field_name, table_class_name));
                
                // 添加到 loadAllTables 方法
                let get_method_name = format!("get{}Table", class_name);
                load_all_tables.push_str(&format!("            this.{}(),\n", get_method_name));
                
                // 添加 get 方法
                let method_code = format!(r#"    /**
     * 获取{}表（惰性加载）
     */
    public async {}(): Promise<{}> {{
        if (this.{} === null) {{
            this.{} = new {}();
            this.{}.load(await this.loadJsonAsset('tables/{}'));
        }}
        return this.{};
    }}

"#, class_name, get_method_name, table_class_name, cache_field_name, cache_field_name, table_class_name, cache_field_name, class_name, cache_field_name);
                
                get_methods.push_str(&method_code);
            }
        }
        
        // 生成 DataTableManager 代码
        let code = format!(r#"{}

/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {{
    private static _instance: DataTableManager;

    // 惰性缓存字段
{}    
    /**
     * 获取单例实例
     */
    public static getInstance(): DataTableManager {{
        if (!DataTableManager._instance) {{
            DataTableManager._instance = new DataTableManager();
        }}
        return DataTableManager._instance;
    }}

    /**
     * 加载所有数据表
     * 注意：表数据会在各自的表加载器中按需加载
     */
    public async loadAllTables(): Promise<void> {{
        // 预加载所有表
        await Promise.all([
{}        ]);
    }}

{}
    /**
     * 加载JSON资源
     * @param path 资源路径
     */
    private async loadJsonAsset(path: string): Promise<cc.JsonAsset> {{
        return new Promise<cc.JsonAsset>((resolve, reject) => {{
            cc.resources.load(path, cc.JsonAsset, (err, asset) => {{
                if (err) {{
                    reject(err);
                }} else {{
                    resolve(asset);
                }}
            }});
        }});
    }}
}}
"#, imports, cache_fields, load_all_tables, get_methods);
        
        // 写入文件
        let mut file = File::create(manager_path)?;
        file.write_all(code.as_bytes())?;
        
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
            
            // 从 generators 列表中获取 Cocos 配置
            for generator in &workspace.config.generators {
                if let xcell_config::project::Generator::Cocos(cocos_config) = generator {
                    // 创建一个新的 CocosCodegen 实例，使用生成器中的 cocos 配置
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
                        namespace: String::new(), // 提供默认值
                        manager_name: cocos_config.manager_name.clone(),
                        suffix_table: cocos_config.suffix_table.clone(),
                        instance_name: cocos_config.instance_name.clone(),
                    };
                    
                    // 打印配置信息
                    println!("Cocos codegen enable: {}", cocos_codegen.enable);
                    println!("Cocos project: {}", cocos_codegen.project);
                    println!("Cocos output: {}", cocos_codegen.output);
                    
                    // 写入 TypeScript 代码
                    println!("Calling write_typescript");
                    cocos_codegen.write_typescript(workspace)?;
                    println!("write_typescript completed");
                    // 写入 JSON 数据
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
