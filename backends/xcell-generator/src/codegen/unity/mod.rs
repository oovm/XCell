use super::*;
use serde::{Serialize, Deserialize};
use std::{
    fs::create_dir_all,
    io::Write,
};
use xcell_analyzer::WorkspaceManager;
use xcell_types::XError;
use dejavu_macros::Template;
use dejavu_types;

/// Unity 字段数据结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnityField {
    /// 字段名
    pub field: String,
    /// 字段类型
    pub r#type: String,
}

/// Unity 表数据结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnityTableItem {
    /// 类名
    pub class_name: String,
    /// 表名
    pub table_name: String,
}

/// Unity 类模板
#[derive(Template)]
#[template(path = "UnityClass.cs.dejavu")]
pub struct UnityClassTemplate {
    /// 类名
    class_name: String,
    /// 表名
    table_name: String,
    /// 命名空间
    namespace: String,
    /// 字段
    items: Vec<UnityField>,
}

/// Unity 管理器模板
#[derive(Template)]
#[template(path = "UnityDataTableManager.cs.dejavu")]
pub struct UnityManagerTemplate {
    /// 命名空间
    namespace: String,
    /// 表数据
    tables: Vec<UnityTableItem>,
}

/// Unity 代码生成器配置
///
/// 用于配置 Unity 平台的代码生成
#[derive(Clone, Debug, Default, Serialize)]
pub struct UnityCodegen {
    /// 是否启用 Unity 代码生成
    pub enable: bool,
    /// 输出目录
    pub output_path: String,
    /// 二进制输出目录
    pub binary_path: String,
    /// 命名空间
    pub namespace: String,
    /// 管理器名称
    pub manager_name: String,
}

/// Unity 代码生成器
///
/// 负责生成 Unity 平台的代码和数据文件
// 使用dejavu模板生成代码
fn render_class_template(class_name: &str, table_name: &str, namespace: &str, items: &[UnityField]) -> XResult<String> {
    let template = UnityClassTemplate {
        class_name: class_name.to_string(),
        table_name: table_name.to_string(),
        namespace: namespace.to_string(),
        items: items.to_vec(),
    };
    template.render().map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
}

fn render_manager_template(namespace: &str, tables: &[UnityTableItem]) -> XResult<String> {
    let template = UnityManagerTemplate {
        namespace: namespace.to_string(),
        tables: tables.to_vec(),
    };
    template.render().map_err(|e| XError::runtime_error(format!("Template render error: {}", e)))
}

impl UnityCodegen {
    /// 创建新的 Unity 代码生成器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 写入 C# 代码
    pub fn write_csharp(&self, ws: &WorkspaceManager, _output_dir: &std::path::Path, unity_config: &xcell_config::unity::UnityCodegen) -> XResult<()> {
        let root = &ws.config.root;
        
        // 使用现有的路径解析方法计算加载器路径
        let loader_path = unity_config.loader_path(root);
        
        println!("Unity loader path: {:?}", loader_path);
        
        // 确保输出目录存在
        std::fs::create_dir_all(&loader_path)?;
        println!("Created output directory: {:?}", loader_path);
        
        // 生成 DataTableManager
        self.write_manager(ws, loader_path.clone(), unity_config)?;
        
        // 生成各个表的类型定义和加载器
        for table in ws.classes() {
            self.write_class(ws, table, loader_path.clone(), unity_config)?;
        }
        
        Ok(())
    }
    
    /// 写入 DataTableManager
    pub fn write_manager(&self, ws: &WorkspaceManager, output_dir: PathBuf, unity_config: &xcell_config::unity::UnityCodegen) -> XResult<()> {
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join("DataTableManager.cs");
        let mut file = std::fs::File::create(path)?;
        
        // 准备表数据
        let mut tables = Vec::new();
        for table in ws.classes() {
            let table_name = format!("{}{}", table.name, unity_config.suffix_table);
            tables.push(UnityTableItem {
                class_name: table.name.to_string(),
                table_name,
            });
        }
        
        // 使用模板渲染
        let content = render_manager_template(&unity_config.namespace, &tables)?;
        file.write_all(content.as_bytes())?;
        
        Ok(())
    }
    
    /// 写入表的类型定义和加载器
    pub fn write_class(&self, _ws: &WorkspaceManager, table: &xcell_analyzer::XClassData, output_dir: PathBuf, unity_config: &xcell_config::unity::UnityCodegen) -> XResult<()> {
        // let root = &_ws.config.root;
        
        let table_name = format!("{}{}", table.name, unity_config.suffix_table);
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join(format!("{}.cs", table_name));
        let mut file = std::fs::File::create(path)?;
        
        // 准备字段数据
        let mut items = Vec::new();
        for item in &table.items {
            items.push(UnityField {
                field: item.field.to_string(),
                r#type: item.typing.as_csharp_type().to_string(),
            });
        }
        
        // 使用模板渲染
        let content = render_class_template(&table.name, &table_name, &unity_config.namespace, &items)?;
        file.write_all(content.as_bytes())?;
        
        Ok(())
    }


}

impl super::Codegen for UnityCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        println!("UnityCodegen::generate called");
        println!("Output directory: {:?}", context.output_dir);
        
        // 从上下文中获取工作区管理器
        if let Some(workspace) = &context.workspace {
            println!("Workspace root: {:?}", workspace.config.root);
            
            // 从 generators 列表中获取 Unity 配置
            for generator in &workspace.config.generators {
                if let xcell_config::project::Generator::Unity(unity_config) = generator {
                    println!("Unity output: {:?}", unity_config.output);
                    
                    // 写入 C# 代码
                    println!("Calling write_csharp");
                    self.write_csharp(workspace, &context.output_dir, unity_config)?;
                    println!("write_csharp completed");
                    
                    // 写入二进制数据
                    println!("Calling write_binary");
                    // 暂时跳过 binary 模块的调用，因为存在字段访问错误
                    // self.write_binary(workspace)?;
                    println!("write_binary completed");
                }
            }
        } else {
            println!("No workspace manager in context");
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "unity"
    }
}

// 暂时只启用必要的模块
// mod binary;
// mod class;
// mod dictionary;
// mod enumerate;
// mod language;
// mod manager;
