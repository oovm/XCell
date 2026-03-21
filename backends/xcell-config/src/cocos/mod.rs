use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use xcell_types::{XError, XResult};

use super::*;

mod der;
mod ser;

/// Cocos 代码生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CocosCodegen {
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
    /// JSON 配置
    pub json: CocosJsonConfig,
}

/// Cocos JSON 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CocosJsonConfig {
    /// 是否启用 JSON 生成
    pub enable: bool,
    /// 生成的 JSON 文件的目录
    pub output: String,
}

impl CocosCodegen {
    /// Cocos 项目文件夹
    pub fn cocos_path(&self, root: &Path) -> XResult<PathBuf> {
        let project = PathBuf::from(&self.project);
        let project = match project.is_absolute() {
            true => project,
            false => root.join(project),
        };
        Ok(project.canonicalize()?)
    }

    /// 生成 TypeScript 代码的文件夹
    pub fn cocos_typescript_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.cocos_path(root)?.join(&self.output);
        let path = dir.join(file_name).with_extension("ts");
        Ok(path)
    }

    /// 生成管理器路径
    pub fn cocos_manager_path(&self, root: &Path) -> XResult<PathBuf> {
        self.cocos_typescript_path(root, &self.manager_name)
    }

    /// 生成 JSON 文件路径
    pub fn cocos_json_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.cocos_path(root)?.join(&self.json.output);
        let path = dir.join(file_name).with_extension("json");
        Ok(path)
    }

    /// 生成 TypeScript 相对路径
    pub fn cocos_ts_relative(&self, file_name: &str) -> String {
        format!("{}/{}.ts", self.output, file_name)
    }

    /// 生成 JSON 相对路径
    pub fn cocos_json_relative(&self, file_name: &str) -> String {
        format!("{}/{}.json", self.json.output, file_name)
    }
    

}
