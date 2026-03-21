use super::*;
use std::{
    fs::{File, create_dir_all},
    path::{Path, PathBuf},
};
use url::Url;
use xcell_analyzer::{WorkspaceManager, XClassData, XDictData, XLanguageData, XListData};
use xcell_types::{ByteOrder, StreamWriter, XCellValue, XResult};

impl UnityCodegen {
    /// 生成二进制产物的文件夹
    pub fn unity_binary_path(&self, root: &Path, file_name: &str) -> XResult<PathBuf> {
        let dir = root.join(&self.storage.binary.output);
        let path = dir.join(file_name).with_extension("bytes");
        Ok(path)
    }
    /// 生成二进制产物的相对路径名
    pub fn unity_bin_relative(&self, file_name: &str) -> String {
        format!("{}/{}.bytes", self.storage.binary.output, file_name)
    }
    pub fn write_binary(&self, ws: &WorkspaceManager) -> XResult<()> {
        // 暂时简化实现，只创建必要的目录结构
        let root = &ws.config.root;
        
        if self.storage.binary.enable {
            let output_dir = PathBuf::from(&self.storage.binary.output);
            let output_dir = match output_dir.is_absolute() {
                true => output_dir,
                false => root.join(output_dir),
            };
            
            if let Some(parent) = output_dir.parent() {
                std::fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }
    fn log_binary(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.unity_binary_path(&ws.config.root, name)?;
        tracing::info!("写入二进制: {}\n{}", self.unity_bin_relative(&name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
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
        (table.mapping.len() as u32).write_to(file, ByteOrder::LittleEndian)?;
        for row in table.mapping.values() {
            for item in &row.data {
                item.write_to(file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }
    pub fn write_list(&self, file: &mut File, table: &XListData) -> XResult<()> {
        (table.mapping.len() as u32).write_to(file, ByteOrder::LittleEndian)?;
        for row in table.mapping.values() {
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
    // fn write_language_table(&self, file: &mut File, table: &BTreeMap<String, String>) -> XResult<()> {
    //     (table.len() as u32).write_to(file, ByteOrder::LittleEndian)?;
    //     for (key, value) in table {
    //         key.write_to(file, ByteOrder::LittleEndian)?;
    //         value.write_to(file, ByteOrder::LittleEndian)?;
    //     }
    //     Ok(())
    // }
}

// 暂时移除这些方法，因为它们依赖于不存在的字段和方法
// impl UnityCodegen {
//     fn write_language_keys(&self, ws: &WorkspaceManager) -> XResult<()> {
//         let mut file = self.log_binary(ws, "LanguageKeys")?;
//         let table = ws.get_language_keys();
//         (table.len() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
//         for key in table {
//             XCellValue::String(key.to_string()).write_to(&mut file, ByteOrder::LittleEndian)?
//         }
//         Ok(())
//     }
//     fn write_language_tables(&self, ws: &WorkspaceManager) {
//         for language in ws.languages() {
//             if let Err(e) = self.write_language_table(ws, &language) {
//                 tracing::error!("write language table {} failed: {}", language.key, e);
//             }
//         }
//     }
//     fn write_language_table(&self, ws: &WorkspaceManager, table: &XLanguageData) -> XResult<()> {
//         let mut file = self.log_binary(ws, &format!("Language{}", table.key))?;
//         (table.localizations.len() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
//         for (key, value) in &table.localizations {
//             XCellValue::String(key.to_string()).write_to(&mut file, ByteOrder::LittleEndian)?;
//             XCellValue::String(value.to_string()).write_to(&mut file, ByteOrder::LittleEndian)?;
//         }
//         Ok(())
//     }
// }
