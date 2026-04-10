use std::{collections::HashMap, path::PathBuf};
use xcell_core::XResult;

/// 代码生成模块
///
/// 支持多种格式的代码和数据生成
pub mod binary;
pub mod cocos;
pub mod dejavu;
pub mod json;
// pub mod readable;
// pub mod sql;
// pub mod typescript;
pub mod unity;
// pub mod xlua;
// pub mod xml;

/// 代码生成上下文
pub struct CodegenContext<'a> {
    /// 输出目录
    pub output_dir: PathBuf,
    /// 配置参数
    pub options: HashMap<String, String>,
    /// 全局配置
    pub global_options: HashMap<String, String>,
    /// 工作区管理器
    pub workspace: Option<&'a xcell_analyzer::WorkspaceManager>,
}

impl<'a> CodegenContext<'a> {
    /// 创建新的代码生成上下文
    pub fn new(output_dir: &str, options: HashMap<String, String>, global_options: HashMap<String, String>) -> Self {
        Self { output_dir: PathBuf::from(output_dir), options, global_options, workspace: None }
    }

    /// 创建带有工作区管理器的代码生成上下文
    pub fn new_with_workspace(output_dir: &str, options: HashMap<String, String>, global_options: HashMap<String, String>, workspace: &'a xcell_analyzer::WorkspaceManager) -> Self {
        Self { output_dir: PathBuf::from(output_dir), options, global_options, workspace: Some(workspace) }
    }

    /// 获取配置参数
    pub fn get_option(&self, key: &str, default: &str) -> String {
        self.options.get(key).unwrap_or(&default.to_string()).clone()
    }

    /// 获取全局配置参数
    pub fn get_global_option(&self, key: &str, default: &str) -> String {
        self.global_options.get(key).unwrap_or(&default.to_string()).clone()
    }

    /// 确保输出目录存在
    pub fn ensure_output_dir(&self) -> XResult<()> {
        std::fs::create_dir_all(&self.output_dir)
            .map_err(|e| crate::error::GeneratorErrorExt::output_dir_error(&format!("{:?}", e)))
    }
}

/// 代码生成器 trait
pub trait Codegen {
    /// 生成代码
    fn generate<'a>(&self, context: &CodegenContext<'a>) -> XResult<()>;

    /// 获取生成器名称
    fn name(&self) -> &'static str;

    /// 初始化生成器
    fn initialize(&mut self) -> XResult<()> {
        Ok(())
    }
}

// 保留原有的CsvCodegen结构
pub struct CsvCodegen {}
