use std::path::PathBuf;

use crate::{WorkspaceManager, XClassData, XClassTable, XDictData, XListData};

use super::*;

impl UnityCodegen {
    /// 生成二进制配置的文件夹
    pub fn unity_binary_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.unity_path(root)?.join(&self.binary.output);
        let path = dir.join(file_name).with_extension("bytes");
        Ok(path)
    }

    pub fn write_binary(&self, ws: &mut WorkspaceManager, root: &Path) -> XResult<()> {
        if let Some(s) = self.unity_binary_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        for list in ws.lists() {
            let file = format!("{}{}", table.name, self.suffix_table);
            let path = self.unity_binary_path(path, &file)?;
        }
    }
    pub fn write_dict_binary(&self, table: &XDictData, path: &Path) -> XResult<()> {
        log::info!("写入二进制: {}\n{}", self.unity_bin_relative(&file), Url::from_file_path(&path)?);
        let cg = CBinaryWriter::default();
        cg.write_list(table, &path)
    }
    pub fn write_class_binary(&self, table: &XClassTable, root: &Path) -> XResult<()> {
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = self.unity_binary_path(root, &file)?;
        log::info!("写入二进制: {}\n{}", self.unity_bin_relative(&file), Url::from_file_path(&path)?);
        let cg = CBinaryWriter::default();
        cg.write_list(table, &path)
    }
}

pub struct CBinaryWriter {}

impl Default for CBinaryWriter {
    fn default() -> Self {
        Self {}
    }
}

impl CBinaryWriter {
    pub fn write_dict(&self, file: &mut File, table: &XDictData) -> XResult<()> {
        (table.map.len() as u32).write_to(file, ByteOrder::LittleEndian)?;
        for row in table.map.values() {
            for item in &row.data {
                item.write_to(file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }
    pub fn write_list(&self, file: &mut File, table: &XListData) -> XResult<()> {
        (table.map.len() as u32).write_to(file, ByteOrder::LittleEndian)?;
        for row in table.map.values() {
            for item in &row.data {
                item.write_to(file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }
    pub fn write_class(&self, file: &mut File, table: &XClassData) -> XResult<()> {
        for item in &table.items {
            item.default.write_to(file, ByteOrder::LittleEndian)?
        }
        Ok(())
    }
}
