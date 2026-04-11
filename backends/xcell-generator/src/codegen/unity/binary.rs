use super::*;
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use url::Url;
use xcell_analyzer::{WorkspaceManager, XClassData, XDictData, XLanguageData, XListData};
use xcell_core::{XCellValue, XResult};
use xcell_provider::{ByteOrder, StreamWriter};

impl UnityGenerator {
    /// 获取二进制产物文件路径
    ///
    /// # 参数
    /// * `root` - 项目根目录
    /// * `file_name` - 文件名
    ///
    /// # 返回值
    /// 二进制文件的完整路径
    pub fn unity_binary_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.config.project_path(root).join(&self.config.storage.as_binary().output);
        let path = dir.join(file_name).with_extension("bytes");
        Ok(path)
    }

    /// 获取二进制产物的相对路径名
    ///
    /// # 参数
    /// * `file_name` - 文件名
    ///
    /// # 返回值
    /// 二进制文件的相对路径字符串
    pub fn unity_bin_relative(&self, file_name: &str) -> String {
        format!("{}/{}.bytes", self.config.storage.as_binary().output, file_name)
    }

    /// 写入所有二进制数据文件
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 操作结果
    pub(super) fn write_binary(&self, ws: &WorkspaceManager) -> XResult<()> {
        if !self.config.storage.as_binary().enable {
            return Ok(());
        }

        let root = &ws.config.root;
        let output_dir = self.config.data_path(root);
        std::fs::create_dir_all(&output_dir)?;

        let writer = CBinaryWriter::default();

        for table in ws.dicts() {
            let mut file = self.log_binary(ws, &table.name)?;
            writer.write_dict(&mut file, table)?;
        }

        for table in ws.lists() {
            let mut file = self.log_binary(ws, &table.name)?;
            writer.write_list(&mut file, table)?;
        }

        for table in ws.classes() {
            let mut file = self.log_binary(ws, &table.name)?;
            writer.write_class(&mut file, table)?;
        }

        self.write_language_binary(ws)?;

        Ok(())
    }

    /// 写入语言键二进制文件
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 操作结果
    fn write_language_keys(&self, ws: &WorkspaceManager) -> XResult<()> {
        let mut file = self.log_binary(ws, "LanguageKeys")?;
        let keys = ws.get_language_keys();
        (keys.len() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
        for key in keys {
            XCellValue::String(key.to_string()).write_to(&mut file, ByteOrder::LittleEndian)?
        }
        Ok(())
    }

    /// 写入所有语言表二进制文件
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 操作结果
    fn write_language_binary(&self, ws: &WorkspaceManager) -> XResult<()> {
        let keys = ws.get_language_keys();
        if keys.is_empty() {
            return Ok(());
        }

        self.write_language_keys(ws)?;

        for language in ws.languages() {
            if let Err(e) = self.write_language_table(ws, &language) {
                tracing::error!("写入语言表 {} 失败: {}", language.key, e);
            }
        }
        Ok(())
    }

    /// 写入单个语言表二进制文件
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `table` - 语言数据
    ///
    /// # 返回值
    /// 操作结果
    fn write_language_table(&self, ws: &WorkspaceManager, table: &XLanguageData) -> XResult<()> {
        let mut file = self.log_binary(ws, &format!("Language{}", table.key))?;
        (table.localizations.len() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
        for (key, value) in &table.localizations {
            XCellValue::String(key.clone()).write_to(&mut file, ByteOrder::LittleEndian)?;
            XCellValue::String(value.clone()).write_to(&mut file, ByteOrder::LittleEndian)?;
        }
        Ok(())
    }

    /// 记录二进制文件创建日志
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `name` - 文件名
    ///
    /// # 返回值
    /// 文件句柄
    fn log_binary(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.unity_binary_path(&ws.config.root, name)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        tracing::info!("写入二进制: {}\n{}", self.unity_bin_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

/// C# 二进制写入器
pub struct CBinaryWriter {}

impl Default for CBinaryWriter {
    fn default() -> Self {
        Self {}
    }
}

impl CBinaryWriter {
    /// 写入字典二进制数据
    ///
    /// # 参数
    /// * `file` - 文件句柄
    /// * `table` - 字典数据表
    ///
    /// # 返回值
    /// 操作结果
    pub fn write_dict(&self, file: &mut File, table: &XDictData) -> XResult<()> {
        (table.mapping.len() as u32).write_to(file, ByteOrder::LittleEndian)?;
        for row in table.mapping.values() {
            for item in &row.data {
                item.write_to(file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }

    /// 写入列表二进制数据
    ///
    /// # 参数
    /// * `file` - 文件句柄
    /// * `table` - 列表数据表
    ///
    /// # 返回值
    /// 操作结果
    pub fn write_list(&self, file: &mut File, table: &XListData) -> XResult<()> {
        (table.mapping.len() as u32).write_to(file, ByteOrder::LittleEndian)?;
        for row in table.mapping.values() {
            for item in &row.data {
                item.write_to(file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }

    /// 写入类二进制数据
    ///
    /// # 参数
    /// * `file` - 文件句柄
    /// * `table` - 类数据表
    ///
    /// # 返回值
    /// 操作结果
    pub fn write_class(&self, file: &mut File, table: &XClassData) -> XResult<()> {
        for item in &table.items {
            item.default.write_to(file, ByteOrder::LittleEndian)?
        }
        Ok(())
    }
}
