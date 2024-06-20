use super::*;
use std::{
    fs::{File, create_dir_all},
    path::Path,
};
use url::Url;
use xcell_analyzer::{UnityCodegen, WorkspaceManager};

/// 暂时禁用 Unity 代码生成，因为存在编译错误
#[derive(Clone, Debug, Default, Serialize)]
pub struct UnityCodegenWrapper {
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

// 暂时禁用以下模块，因为存在编译错误
// mod binary;
// mod class;
// mod dictionary;
// mod enumerate;
// mod language;
// mod manager;

// 暂时移除 UnityCodegen 的实现，因为存在编译错误

impl super::Codegen for UnityCodegenWrapper {
    fn generate(&self, _context: &super::CodegenContext) -> XResult<()> {
        // TODO: Implement Unity code generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "unity"
    }
}
