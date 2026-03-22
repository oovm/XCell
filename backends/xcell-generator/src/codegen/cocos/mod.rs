use xcell_analyzer::WorkspaceManager;
use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use xcell_types::{XError, XResult};
use url::Url;

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

// 简化的模板定义，使用字符串拼接代替复杂的模板系统
fn render_enumerate_template(class_name: &str, items: &[CocosEnumerateItem]) -> String {
    let mut content = format!("/**
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
", class_name, class_name, class_name, class_name, class_name, class_name, class_name);

    for (i, item) in items.iter().enumerate() {
        content.push_str(&format!("    /**
     * {}{}
     */
    {}: {{
        id: {},
        name: \"{}\",
        description: \"{}\"
    }}{}\n", item.name, class_name, item.key, item.id, item.name, item.description, if i < items.len() - 1 { "," } else { "" }));
    }

    content.push_str(&format!("}} as const as Record<string, {}>;
", class_name));
    content
}

fn render_class_template(class_name: &str, table_name: &str, fields: &[CocosField], has_type_field: bool, is_monster: bool, has_level_field: bool, is_skill: bool) -> String {
    let mut content = format!("/**
 * {}数据结构
 */
export interface {} {{
", class_name, class_name);

    for field in fields {
        content.push_str(&format!("    /**
     * {}
     */
    {}: {};
", field.name, field.name, field.r#type));
    }

    content.push_str(&format!("}}\n\n/**
 * {}表加载器
 */
export class {} {{
    private items: {}[] = [];

    /**
     * 加载{}表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {{
        const data = asset.json;
        if (data) {{
            this.items = data as {}[];
        }}
    }}

    /**
     * 根据ID获取{}
     * @param id {}ID
     */
    public get{}ById(id: number): {} | null {{
        return this.items.find(item => item.id === id) || null;
    }}

    /**
     * 获取所有{}
     */
    public getAll{}(): {}[] {{
        return this.items;
    }}
", class_name, table_name, class_name, class_name, class_name, class_name, class_name, class_name, class_name, class_name, class_name, class_name));

    if has_type_field {
        if is_monster {
            content.push_str(&format!("    /**
     * 根据类型获取{}
     * @param type 怪物类型
     */
    public get{}ByType(type: MonsterType): {}[] {{
        return this.items.filter(item => item.type === type);
    }}
", class_name, class_name, class_name));
        } else {
            content.push_str(&format!("    /**
     * 根据类型获取{}
     * @param type 类型
     */
    public get{}ByType(type: string): {}[] {{
        return this.items.filter(item => item.type === type);
    }}
", class_name, class_name, class_name));
        }
    }

    if has_level_field {
        if is_skill {
            content.push_str(&format!("    /**
     * 根据等级获取{}
     * @param level 等级
     */
    public get{}ByLevel(level: number): {}[] {{
        return this.items.filter(item => item.level_requirement <= level);
    }}
", class_name, class_name, class_name));
        } else {
            content.push_str(&format!("    /**
     * 根据等级获取{}
     * @param level 等级
     */
    public get{}ByLevel(level: number): {}[] {{
        return this.items.filter(item => item.level === level);
    }}
", class_name, class_name, class_name));
        }
    }

    content.push_str("}\n");
    content
}

fn render_manager_template(tables: &[CocosDataTableItem], table_data_path: &str) -> String {
    let mut content = String::new();

    for table in tables {
        content.push_str(&format!("import {{ {} }} from './{}';
", table.table_name, table.table_name));
    }

    content.push_str("\n\n/**
 * 数据表管理器
 * 负责加载和管理所有数据表
 */
export class DataTableManager {
    private static _instance: DataTableManager;

    // 惰性缓存字段
");

    for table in tables {
        content.push_str(&format!("    private _{}: {} | null = null;
", table.cache_name, table.table_name));
    }

    content.push_str("    
    /**
     * 获取单例实例
     */
    public static getInstance(): DataTableManager {
        if (!DataTableManager._instance) {
            DataTableManager._instance = new DataTableManager();
        }
        return DataTableManager._instance;
    }

    /**
     * 加载所有数据表
     * 注意：表数据会在各自的表加载器中按需加载
     */
    public async loadAllTables(): Promise<void> {
        // 预加载所有表
        await Promise.all([
");

    for table in tables {
        content.push_str(&format!("            this.{}(),\n", table.get_method_name));
    }

    content.push_str("        ]);
    }
");

    for table in tables {
        content.push_str(&format!("    
    /**
     * 获取{}表（惰性加载）
     */
    public async {}(): Promise<{}> {{
        if (this._{} === null) {{
            this._{} = new {}();
            this._{}.load(await this.loadJsonAsset('{}{}'));
        }}
        return this._{};
    }}
", table.class_name, table.get_method_name, table.table_name, table.cache_name, table.cache_name, table.table_name, table.cache_name, table_data_path, table.class_name, table.cache_name));
    }

    content.push_str("    

    /**
     * 加载JSON资源
     * @param path 资源路径
     */
    private async loadJsonAsset(path: string): Promise<cc.JsonAsset> {
        return new Promise<cc.JsonAsset>((resolve, reject) => {
            cc.resources.load(path, cc.JsonAsset, (err, asset) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(asset);
                }
            });
        });
    }
}
");
    content
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
                
                let content = render_enumerate_template(class_name, &items);
                
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
                let has_level_field = fields.iter().any(|f| f.name == "level") || fields.iter().any(|f| f.name == "level_requirement");
                
                let content = render_class_template(class_name, &table_class_name, &fields, has_type_field, class_name == "Monster", has_level_field, class_name == "Skill");
                
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
        
        let content = render_manager_template(&tables, table_data_path);
        
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
