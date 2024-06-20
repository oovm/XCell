use crate::{WorkspaceManager, XCellHeader, XClassData, XDictData, XEnumerateData, XListData};
use dejavu_derive::Template;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};
use xcell_errors::{XError, XResult, for_3rd::Url};

mod class;
mod dictionary;
mod enumerate;
mod manager;

/// Cocos 代码生成器配置
///
/// 用于配置 Cocos 平台的代码生成
#[derive(Clone, Debug, Default, Serialize)]
pub struct CocosCodegen {
    /// 是否启用 Cocos 代码生成
    pub enable: bool,
    /// Cocos 项目目录，推荐使用相对路径
    pub project: String,
    /// 输出目录
    pub output: String,
    /// 生成代码的命名空间
    pub namespace: String,
    /// 生成的管理器名称
    pub manager_name: String,
    /// 生成的表名后缀
    pub suffix_table: String,
    /// 生成的实例名称
    pub instance_name: String,
    /// JSON 配置
    pub json: CocosJsonConfig,
}

/// Cocos JSON 配置
///
/// 用于配置 JSON 数据生成
#[derive(Clone, Debug, Default, Serialize)]
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
        let project = PathBuf::from(&self.project);
        let project = match project.is_absolute() {
            true => project,
            false => root.join(project),
        };
        Ok(project.canonicalize()?)
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
        let dir = self.cocos_path(root)?.join(&self.output);
        let path = dir.join(file_name).with_extension("ts");
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
        let dir = self.cocos_path(root)?.join(&self.json.output);
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
        format!("{}/{}.json", self.json.output, file_name)
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
            if self.json.enable {
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
        if !self.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

        for table in ws.classes() {
            if let Err(e) = self.write_class(ws, table) {
                tracing::error!("生成Cocos类失败: {}", e);
            }
        }
        for table in ws.enumerates() {
            if let Err(e) = self.write_enumerate(ws, table) {
                tracing::error!("生成Cocos枚举失败: {}", e);
            }
        }
        for table in ws.dicts() {
            if let Err(e) = self.write_dict(ws, table) {
                tracing::error!("生成Cocos字典失败: {}", e);
            }
        }
        for table in ws.lists() {
            if let Err(e) = self.write_list(ws, table) {
                tracing::error!("生成Cocos列表失败: {}", e);
            }
        }

        self.write_manager(ws)?;
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
        if !self.enable || !self.json.enable {
            return Ok(());
        }

        self.ensure_path(&ws.config.root)?;

        // TODO: Implement JSON data generation
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
        // TODO: Implement Cocos code generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "cocos"
    }
}
