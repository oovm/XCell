use std::path::PathBuf;

use crate::{WorkspaceManager, XClassData, XDictData, XListData};

use super::*;

impl UnityCodegen {
    /// 生成二进制产物的文件夹
    pub fn unity_binary_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = self.unity_path(root)?.join(&self.binary.output);
        let path = dir.join(file_name).with_extension("bytes");
        Ok(path)
    }
    /// 生成二进制产物的相对路径名
    pub fn unity_bin_relative(&self, file_name: &str) -> String {
        format!("{}/{}.bytes", self.binary.output, file_name)
    }

    pub fn write_binary(&self, ws: &WorkspaceManager) -> XResult<()> {
        if let Some(s) = self.unity_binary_path(&ws.config.root, "test")?.parent() {
            create_dir_all(s)?
        }
        let mut w = CBinaryWriter::default();
        for list in ws.lists() {
            if let Err(e) = self.log_write(ws, &list.name).and_then(|mut o| w.write_list(&mut o, list)) {
                log::error!("write list {} failed: {}", list.name, e);
            }
        }
        for dict in ws.dicts() {
            if let Err(e) = self.log_write(ws, &dict.name).and_then(|mut o| w.write_dict(&mut o, dict)) {
                log::error!("write list {} failed: {}", dict.name, e);
            }
        }

        Ok(())
    }
    fn log_write(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let file = format!("{}{}", name, self.suffix_table);
        let path = self.unity_binary_path(&ws.config.root, &file)?;
        log::info!("写入二进制: {}\n{}", self.unity_bin_relative(&file), Url::from_file_path(&path)?);
        return Ok(File::create(path)?);
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
