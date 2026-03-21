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
    /// 加载器配置
    pub loader: CocosLoader,
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
        // 优先使用 cocos 子目录
        let cocos_subdir = root.join("cocos");
        if cocos_subdir.exists() {
            println!("Using cocos subdirectory: {:?}", cocos_subdir);
            return Ok(cocos_subdir);
        }
        
        // 如果 cocos 子目录不存在，使用配置中的 project 路径
        let project = PathBuf::from(&self.loader.project);
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
        let output_path = if self.loader.output.is_empty() {
            // 默认输出到 assets/scripts/dataTable/generated 目录
            cocos_path.join("assets").join("scripts").join("dataTable").join("generated")
        } else {
            cocos_path.join(&self.loader.output)
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
        let dir = self.cocos_path(root)?.join(&self.storage.json.output);
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
        self.cocos_typescript_path(root, &self.loader.manager_name)
    }

    /// 获取 TypeScript 相对路径
    ///
    /// # 参数
    /// * `file_name` - 文件名
    ///
    /// # 返回值
    /// 返回 TypeScript 文件的相对路径。
    pub fn cocos_ts_relative(&self, file_name: &str) -> String {
        format!("{}/{}.ts", self.loader.output, file_name)
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
        if self.loader.enable {
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
        if let Some(s) = self.cocos_typescript_path(root, "test")?.parent() {
            std::fs::create_dir_all(s)?;
        }
        
        // 从工作区获取表数据
        let list_tables = ws.lists();
        
        println!("Found {} list tables", list_tables.count());
        
        // 重置迭代器
        let list_tables = ws.lists();
        
        // 处理列表表
        for list_data in list_tables {
            let class_name = &list_data.name;
            let table_class_name = format!("{}Table", class_name);
            let ts_path = self.cocos_typescript_path(root, &table_class_name)?;
            
            println!("Processing list table: {} -> {}", class_name, ts_path.display());
            
            // 创建目录
            if let Some(parent) = ts_path.parent() {
                std::fs::create_dir_all(parent)?;
                println!("Created directory: {:?}", parent);
            }
            
            // 准备字段数据
            let mut fields = Vec::new();
            let mut has_type_field = false;
            let mut has_level_field = false;
            
            // 从表结构获取字段信息
            for header in &list_data.headers {
                let field_name = &header.field_name;
                let field_type = &header.typing;
                
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
            
            // 读取模板文件
            let template_path = Path::new(&ws.config.root).join("backends").join("xcell-generator").join("templates").join("BuildCocosClass.ts.dejavu");
            println!("Reading template from: {:?}", template_path);
            let template_content = std::fs::read_to_string(template_path)?;
            println!("Template content length: {}", template_content.len());
            
            // 替换模板变量
            let mut code = template_content
                .replace("{{ class_name }}", &class_name)
                .replace("{{ table_name }}", &table_class_name);
            println!("Code after initial replacement: {}", code);
            
            // 添加 MonsterType 导入
            if class_name == "Monster" {
                code = format!("import {{ MonsterType }} from \"./MonsterType\";\n\n{}", code);
            }
            
            // 生成字段代码
            let mut fields_code = String::new();
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
                    // 处理枚举类型
                    "MonsterType"
                } else {
                    &field.r#type
                };
                
                fields_code.push_str(&format!("    /**
     * {}
     */
    {}: {};\n", field.name, field.name, field_type));
            }
            code = code.replace("<%- for field in fields %>
    /**
     * {{ field.name }}
     */
    {{ field.name }}: {{ field.type }};
<%- endfor %>", &fields_code);
            
            // 处理条件代码
            if has_type_field {
                let type_method = format!("    /**
     * 根据类型获取{}
     * @param type 类型
     */
    public get{}ByType(type: string): {}[] {{\n        return this.items.filter(item => item.type === type);\n    }}\n", class_name, class_name, class_name);
                code = code.replace("<% if has_type_field %>
    /**
     * 根据类型获取{{ class_name }}
     * @param type 类型
     */
    public get{{ class_name }}ByType(type: string): {{ class_name }}[] {
        return this.items.filter(item => item.type === type);
    }
<% endif %>", &type_method);
            } else {
                code = code.replace("<% if has_type_field %>
    /**
     * 根据类型获取{{ class_name }}
     * @param type 类型
     */
    public get{{ class_name }}ByType(type: string): {{ class_name }}[] {
        return this.items.filter(item => item.type === type);
    }
<% endif %>", "");
            }
            
            if has_level_field {
                let level_method = format!("    /**
     * 根据等级获取{}
     * @param level 等级
     */
    public get{}ByLevel(level: number): {}[] {{\n        return this.items.filter(item => item.level === level);\n    }}\n", class_name, class_name, class_name);
                code = code.replace("<% if has_level_field %>
    /**
     * 根据等级获取{{ class_name }}
     * @param level 等级
     */
    public get{{ class_name }}ByLevel(level: string): {{ class_name }}[] {
        return this.items.filter(item => item.level === level);
    }
<% endif %>", &level_method);
            } else {
                code = code.replace("<% if has_level_field %>
    /**
     * 根据等级获取{{ class_name }}
     * @param level 等级
     */
    public get{{ class_name }}ByLevel(level: string): {{ class_name }}[] {
        return this.items.filter(item => item.level === level);
    }
<% endif %>", "");
            }
            
            // 特殊处理 SkillsTable 的 getSkillsByLevel 方法
            if class_name == "Skill" {
                code = code.replace("    /**
     * 根据等级获取Skill
     * @param level 等级
     */
    public getSkillByLevel(level: number): Skill[] {
        return this.items.filter(item => item.level === level);
    }
", "    /**
     * 根据等级获取Skill
     * @param level 等级
     */
    public getSkillsByLevel(level: number): Skill[] {
        return this.items.filter(item => item.level_requirement <= level);
    }
");
            }
            
            // 修改 load 方法，使其与参考效果一致
            let load_method = format!("    /**
     * 加载{}表数据
     * @param asset JSON资源
     */
    public load(asset: cc.JsonAsset): void {{\n        const data = asset.json;\n        if (data) {{\n            this.items = data as {}[];\n        }}\n    }}", class_name, class_name);
            code = code.replace("    /**
     * 加载{{ class_name }}表数据
     */
    public async load(): Promise<void> {
        const path = 'tables/{{ class_name }}';
        const asset = await new Promise<cc.JsonAsset>((resolve, reject) => {
            cc.resources.load(path, cc.JsonAsset, (err, asset) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(asset);
                }
            });
        });

        const data = asset.json;
        if (data) {
            this.items = data;
        }
    }", &load_method);
            
            // 修改 get{{ class_name }}ById 方法，使其与参考效果一致
            let get_by_id_method = format!("    /**
     * 根据ID获取{}
     * @param id {}ID
     */
    public get{}ById(id: number): {} | null {{\n        return this.items.find(item => item.id === id) || null;\n    }}", class_name, class_name, class_name, class_name);
            code = code.replace("    /**
     * 根据ID获取{{ class_name }}
     * @param id {{ class_name }}ID
     */
    public get{{ class_name }}ById(id: string): {{ class_name }} | null {
        return this.items.find(item => item.id === id) || null;
    }", &get_by_id_method);
            
            // 修改 getAll{{ class_name }} 方法
            let get_all_method = format!("    /**
     * 获取所有{}
     */
    public getAll{}(): {}[] {{\n        return this.items;\n    }}", class_name, class_name, class_name);
            code = code.replace("    /**
     * 获取所有{{ class_name }}
     */
    public getAll{{ class_name }}(): {{ class_name }}[] {
        return this.items;
    }", &get_all_method);
            
            // 特殊处理 MonstersTable 的 getMonstersByType 方法
            if class_name == "Monster" {
                code = code.replace("    /**
     * 根据类型获取Monster
     * @param type 类型
     */
    public getMonsterByType(type: string): Monster[] {{\n        return this.items.filter(item => item.type === type);\n    }}\n", "    /**
     * 根据类型获取Monster
     * @param type 怪物类型
     */
    public getMonstersByType(type: MonsterType): Monster[] {{\n        return this.items.filter(item => item.type === type);\n    }}\n");
            }
            
            // 写入文件
            let mut file = File::create(ts_path)?;
            file.write_all(code.as_bytes())?;
            
            println!("Created TypeScript file for {} successfully", class_name);
        }
        
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
        if !self.loader.enable || !self.storage.json.enable {
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
            
            // 创建一个新的 CocosCodegen 实例，使用项目配置中的 cocos 配置
            let cocos_codegen = CocosCodegen {
                storage: CocosStorage {
                    json: CocosJsonConfig {
                        enable: workspace.config.cocos.storage.json.enable,
                        output: workspace.config.cocos.storage.json.output.clone(),
                    },
                },
                loader: CocosLoader {
                    enable: workspace.config.cocos.loader.enable,
                    project: workspace.config.cocos.loader.project.clone(),
                    output: workspace.config.cocos.loader.output.clone(),
                    namespace: workspace.config.cocos.loader.namespace.clone(),
                    manager_name: workspace.config.cocos.loader.manager_name.clone(),
                    suffix_table: workspace.config.cocos.loader.suffix_table.clone(),
                    instance_name: workspace.config.cocos.loader.instance_name.clone(),
                },
            };
            
            // 打印配置信息
            println!("Cocos codegen enable: {}", cocos_codegen.loader.enable);
            println!("Cocos project: {}", cocos_codegen.loader.project);
            println!("Cocos output: {}", cocos_codegen.loader.output);
            
            // 写入 TypeScript 代码
            println!("Calling write_typescript");
            cocos_codegen.write_typescript(workspace)?;
            println!("write_typescript completed");
            // 写入 JSON 数据
            println!("Calling write_json");
            cocos_codegen.write_json(workspace)?;
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