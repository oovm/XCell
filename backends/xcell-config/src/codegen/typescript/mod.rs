use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use xcell_core::{XError, XResult};

use super::*;

mod der;
mod ser;

/// TypeScript 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptCodegen {
    /// Whether to generate TypeScript code
    pub enable: bool,
    /// TypeScript 项目的工作目录, 建议使用相对路径
    pub project: String,
    /// Output directory
    pub output: String,
    /// 数据文件的输出目录，以 `project` 为根目录
    pub storage: String,
    /// 运行时加载类型 (json)
    pub storage_type: String,
    /// Generated manager name
    pub manager_name: String,
    /// Generated table name suffix
    pub suffix_table: String,
    /// Generated instance name
    pub instance_name: String,
    /// 使用 dejavu 模板生成代码的模板目录，空字符串表示使用默认模板
    pub loader_template: String,
    /// 是否跳过 Manager 生成
    #[serde(default)]
    pub skip_manager: bool,
}

impl TypeScriptCodegen {
    /// 解析 TypeScript 项目的工作目录
    pub fn project_path(&self, root: &Path) -> XResult<PathBuf> {
        let project = PathBuf::from(&self.project);
        let project = match project.is_absolute() {
            true => project,
            false => root.join(project),
        };
        Ok(project.canonicalize()?)
    }

    /// 解析 TypeScript 代码文件的完整路径
    pub fn ts_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.project_path(root)?.join(&self.output);
        let path = dir.join(file_name).with_extension("ts");
        Ok(path)
    }

    /// 解析 TypeScript 管理器代码文件的完整路径
    pub fn ts_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.ts_path(root, &self.manager_name)
    }

    /// 解析数据文件的输出目录
    pub fn data_path(&self, root: &Path) -> XResult<PathBuf> {
        if self.storage.is_empty() {
            Ok(self.project_path(root)?.join(&self.output))
        } else {
            Ok(self.project_path(root)?.join(&self.storage))
        }
    }
}
