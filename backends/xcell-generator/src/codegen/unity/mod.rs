use super::*;
use xcell_analyzer::WorkspaceManager;
use xcell_config::UnityCodegen;
use xcell_core::XResult;

mod binary;
mod class;
mod dictionary;
mod enumerate;
mod language;
mod manager;

/// Unity 代码生成器
///
/// 封装 `UnityCodegen` 配置，提供 Unity 平台的 C# 代码生成功能。
pub struct UnityGenerator {
    /// Unity 代码生成配置
    config: UnityCodegen,
}

impl UnityGenerator {
    /// 创建新的 Unity 代码生成器
    ///
    /// # 参数
    /// * `config` - Unity 代码生成配置
    ///
    /// # 返回值
    /// Unity 代码生成器实例
    pub fn new(config: UnityCodegen) -> Self {
        Self { config }
    }

    /// 从默认配置创建 Unity 代码生成器
    ///
    /// # 返回值
    /// 使用默认配置的 Unity 代码生成器实例
    pub fn default_generator() -> Self {
        Self { config: UnityCodegen::default() }
    }

    /// 获取配置引用
    ///
    /// # 返回值
    /// Unity 代码生成配置的引用
    pub fn config(&self) -> &UnityCodegen {
        &self.config
    }
}

impl Codegen for UnityGenerator {
    fn name(&self) -> &'static str {
        "unity"
    }

    fn generate<'a>(&self, context: &CodegenContext<'a>) -> XResult<()> {
        self.config.validate().map_err(|e| xcell_core::XError::runtime_error(e))?;

        match context.workspace {
            Some(ws) => {
                self.write_csharp(ws)?;
                self.write_binary(ws)?;
                Ok(())
            }
            None => Err(xcell_core::XError::runtime_error("Unity 代码生成需要 WorkspaceManager")),
        }
    }
}

impl UnityGenerator {
    /// 写入所有 C# 代码文件
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 操作结果
    fn write_csharp(&self, ws: &WorkspaceManager) -> XResult<()> {
        for table in ws.classes() {
            if let Err(e) = self.write_class(ws, table) {
                tracing::error!("写入类 {} 失败: {}", table.name, e);
            }
        }

        for table in ws.enumerates() {
            if let Err(e) = self.write_enumerate(ws, table) {
                tracing::error!("写入枚举 {} 失败: {}", table.name, e);
            }
        }

        for table in ws.dicts() {
            if let Err(e) = self.write_dict(ws, table) {
                tracing::error!("写入字典 {} 失败: {}", table.name, e);
            }
        }

        for table in ws.lists() {
            if let Err(e) = self.write_list(ws, table) {
                tracing::error!("写入列表 {} 失败: {}", table.name, e);
            }
        }

        if !ws.languages().is_empty() {
            if let Err(e) = self.write_language(ws) {
                tracing::error!("写入语言表失败: {}", e);
            }
        }

        if let Err(e) = self.write_manager(ws) {
            tracing::error!("写入管理器失败: {}", e);
        }

        Ok(())
    }
}
