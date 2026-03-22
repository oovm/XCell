use super::*;
use serde::{Serialize, Deserialize};
use xcell_analyzer::WorkspaceManager;
use xcell_config::UnityCodegen;
use xcell_types::XError;

mod class;
mod dictionary;
mod enumerate;
mod language;
mod manager;

pub use class::*;
pub use enumerate::*;
pub use manager::*;

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

impl UnityCodegen {
    /// 写入 C# 代码
    pub fn write_csharp(&self, ws: &WorkspaceManager) -> XResult<()> {
        let loader_path = self.loader_path(&ws.config.root);
        
        println!("Unity loader path: {:?}", loader_path);
        
        std::fs::create_dir_all(&loader_path)?;
        println!("Created output directory: {:?}", loader_path);
        
        self.write_manager(ws)?;
        
        for table in ws.classes() {
            self.write_class(ws, table)?;
        }
        
        for table in ws.enumerates() {
            self.write_enumerate(ws, table)?;
        }
        
        Ok(())
    }
}

impl super::Codegen for UnityCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        println!("UnityCodegen::generate called");
        println!("Output directory: {:?}", context.output_dir);
        
        if let Some(workspace) = &context.workspace {
            println!("Workspace root: {:?}", workspace.config.root);
            
            println!("Unity output: {:?}", self.output);
            
            println!("Calling write_csharp");
            self.write_csharp(workspace)?;
            println!("write_csharp completed");
        } else {
            println!("No workspace manager in context");
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "unity"
    }
}
