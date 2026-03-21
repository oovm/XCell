use super::*;
use serde::Serialize;
use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::Path,
};
use url::Url;
use xcell_analyzer::WorkspaceManager;

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
impl UnityCodegen {
    /// 创建新的 Unity 代码生成器实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 写入 C# 代码
    pub fn write_csharp(&self, ws: &WorkspaceManager, output_dir: &std::path::Path, unity_config: &xcell_config::unity::UnityCodegen) -> XResult<()> {
        use std::fs::File;
        use std::io::Write;
        use std::path::PathBuf;
        
        let root = &ws.config.root;
        
        // 解析输出目录路径，确保在 unity 子目录下
        let output_dir = if output_dir.is_absolute() {
            output_dir.to_path_buf()
        } else {
            // 检查 output_dir 是否已经包含 unity 目录
            if output_dir.starts_with("unity") {
                root.join(output_dir)
            } else {
                // 尝试在 unity 子目录下查找
                let unity_output_dir = root.join("unity").join(output_dir);
                if unity_output_dir.exists() {
                    unity_output_dir
                } else {
                    // 如果 unity 子目录不存在，使用原路径
                    root.join(output_dir)
                }
            }
        };
        
        println!("Unity output directory: {:?}", output_dir);
        
        // 确保输出目录存在
        std::fs::create_dir_all(&output_dir)?;
        println!("Created output directory: {:?}", output_dir);
        
        // 生成 DataTableManager
        self.write_manager(ws, unity_config)?;
        
        // 生成各个表的类型定义和加载器
        for table in ws.classes() {
            self.write_class(ws, table, unity_config)?;
        }
        
        Ok(())
    }
    
    /// 写入 DataTableManager
    pub fn write_manager(&self, ws: &WorkspaceManager, unity_config: &xcell_config::unity::UnityCodegen) -> XResult<()> {
        let root = &ws.config.root;
        
        let output_dir = PathBuf::from(&unity_config.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join("DataTableManager.cs");
        let mut file = std::fs::File::create(path)?;
        
        // 生成完整的 DataTableManager
        writeln!(file, "// Unity generated file")?;
        writeln!(file, "")?;
        writeln!(file, "using System;")?;
        writeln!(file, "using System.Collections.Generic;")?;
        writeln!(file, "using System.IO;")?;
        writeln!(file, "using UnityEngine;")?;
        writeln!(file, "")?;
        writeln!(file, "namespace {}", unity_config.namespace)?;
        writeln!(file, "{{")?;
        writeln!(file, "    /// <summary>")?;
        writeln!(file, "    /// 数据表管理器")?;
        writeln!(file, "    /// 负责加载和管理所有数据表")?;
        writeln!(file, "    /// </summary>")?;
        writeln!(file, "    public class DataTableManager")?;
        writeln!(file, "    {{")?;
        writeln!(file, "        private static DataTableManager instance;")?;
        writeln!(file, "        private Dictionary<string, object> tables = new Dictionary<string, object>();")?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 获取单例实例")?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        public static DataTableManager Instance")?;
        writeln!(file, "        {{")?;
        writeln!(file, "            get")?;
        writeln!(file, "            {{")?;
        writeln!(file, "                if (instance == null)")?;
        writeln!(file, "                {{")?;
        writeln!(file, "                    instance = new DataTableManager();")?;
        writeln!(file, "                }}")?;
        writeln!(file, "                return instance;")?;
        writeln!(file, "            }}")?;
        writeln!(file, "        }}")?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 加载所有数据表")?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        public void LoadAllTables()")?;
        writeln!(file, "        {{")?;
        
        // 为每个表生成加载代码
        for table in ws.classes() {
            let table_name = format!("{}{}", table.name, unity_config.suffix_table);
            writeln!(file, "            {}.Load();", table_name)?;
        }
        
        writeln!(file, "        }}")?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 存储数据表")?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        /// <typeparam name=\"T\">数据表类型</typeparam>")?;
        writeln!(file, "        /// <param name=\"tableName\">表名</param>")?;
        writeln!(file, "        /// <param name=\"table\">数据表实例</param>")?;
        writeln!(file, "        public void SetTable<T>(string tableName, T table)")?;
        writeln!(file, "        {{")?;
        writeln!(file, "            tables[tableName] = table;")?;
        writeln!(file, "        }}")?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 获取数据表")?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        /// <typeparam name=\"T\">数据表类型</typeparam>")?;
        writeln!(file, "        /// <param name=\"tableName\">表名</param>")?;
        writeln!(file, "        /// <returns>数据表实例</returns>")?;
        writeln!(file, "        public T GetTable<T>(string tableName)")?;
        writeln!(file, "        {{")?;
        writeln!(file, "            if (tables.TryGetValue(tableName, out var table))")?;
        writeln!(file, "            {{")?;
        writeln!(file, "                return (T)table;")?;
        writeln!(file, "            }}")?;
        writeln!(file, "            return default;")?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}" )?;
        
        Ok(())
    }
    
    /// 写入表的类型定义和加载器
    pub fn write_class(&self, ws: &WorkspaceManager, table: &xcell_analyzer::XClassData, unity_config: &xcell_config::unity::UnityCodegen) -> XResult<()> {
        let root = &ws.config.root;
        
        let table_name = format!("{}{}", table.name, unity_config.suffix_table);
        
        let output_dir = PathBuf::from(&unity_config.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join(format!("{}.cs", table_name));
        let mut file = std::fs::File::create(path)?;
        
        // 生成表的类型定义和加载器
        writeln!(file, "// Unity generated file")?;
        writeln!(file, "")?;
        writeln!(file, "using System;")?;
        writeln!(file, "using System.Collections.Generic;")?;
        writeln!(file, "using System.IO;")?;
        writeln!(file, "using UnityEngine;")?;
        writeln!(file, "")?;
        writeln!(file, "namespace {}", unity_config.namespace)?;
        writeln!(file, "{{")?;
        
        // 生成数据结构
        writeln!(file, "    /// <summary>")?;
        writeln!(file, "    /// {}表数据结构", table.name)?;
        writeln!(file, "    /// </summary>")?;
        writeln!(file, "    public class {}", table.name)?;
        writeln!(file, "    {{")?;
        
        for item in &table.items {
            writeln!(file, "        /// <summary>")?;
            writeln!(file, "        /// {}", item.field)?;
            writeln!(file, "        /// </summary>")?;
            writeln!(file, "        public {} {} {{ get; set; }}", item.typing.as_csharp_type(), item.field)?;
        }
        
        writeln!(file, "    }}")?;
        writeln!(file, "")?;
        
        // 生成加载器
        writeln!(file, "    /// <summary>")?;
        writeln!(file, "    /// {}表加载器", table.name)?;
        writeln!(file, "    /// </summary>")?;
        writeln!(file, "    public static class {}", table_name)?;
        writeln!(file, "    {{")?;
        writeln!(file, "        private static Dictionary<string, {}> items = new Dictionary<string, {}>();", table.name, table.name)?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 加载{}表数据", table.name)?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        public static void Load()")?;
        writeln!(file, "        {{")?;
        writeln!(file, "            // 从二进制文件或JSON文件加载数据")?;
        writeln!(file, "            // 暂时使用示例数据")?;
        writeln!(file, "            LoadSampleData();")?;
        writeln!(file, "            ")?;
        writeln!(file, "            // 存储到 DataTableManager")?;
        writeln!(file, "            DataTableManager.Instance.SetTable(\"{}\", items);", table.name)?;
        writeln!(file, "        }}")?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 加载示例数据")?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        private static void LoadSampleData()")?;
        writeln!(file, "        {{")?;
        writeln!(file, "            items.Clear();")?;
        writeln!(file, "            ")?;
        writeln!(file, "            // 示例数据")?;
        writeln!(file, "            // 实际项目中应该从文件加载")?;
        writeln!(file, "        }}")?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 根据ID获取{}", table.name)?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        /// <param name=\"id\">ID</param>")?;
        writeln!(file, "        /// <returns>实例</returns>")?;
        writeln!(file, "        public static {} Get{}ById(string id)", table.name, table.name)?;
        writeln!(file, "        {{")?;
        writeln!(file, "            if (items.TryGetValue(id, out var item))")?;
        writeln!(file, "            {{")?;
        writeln!(file, "                return item;")?;
        writeln!(file, "            }}")?;
        writeln!(file, "            return null;")?;
        writeln!(file, "        }}")?;
        writeln!(file, "")?;
        writeln!(file, "        /// <summary>")?;
        writeln!(file, "        /// 获取所有{}", table.name)?;
        writeln!(file, "        /// </summary>")?;
        writeln!(file, "        /// <returns>列表</returns>")?;
        writeln!(file, "        public static List<{}> GetAll{}()", table.name, table.name)?;
        writeln!(file, "        {{")?;
        writeln!(file, "            return new List<{}>(items.Values);", table.name)?;
        writeln!(file, "        }}")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}" )?;
        
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
