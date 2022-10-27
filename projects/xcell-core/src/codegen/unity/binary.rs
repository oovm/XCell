use crate::XLanguageData;

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
        let w = CBinaryWriter::default();
        for class in ws.class_data() {
            if let Err(e) = self
                .log_binary(ws, &format!("{}{}", class.name, ws.config.unity.suffix_table))
                .and_then(|mut o| w.write_class(&mut o, class))
            {
                log::error!("write class {} failed: {}", class.name, e);
            }
        }
        for list in ws.lists() {
            if let Err(e) = self
                .log_binary(ws, &format!("{}{}", list.name, ws.config.unity.suffix_table))
                .and_then(|mut o| w.write_list(&mut o, list))
            {
                log::error!("write list {} failed: {}", list.name, e);
            }
        }
        for dict in ws.dicts() {
            if let Err(e) = self
                .log_binary(ws, &format!("{}{}", dict.name, ws.config.unity.suffix_table))
                .and_then(|mut o| w.write_dict(&mut o, dict))
            {
                log::error!("write dict {} failed: {}", dict.name, e);
            }
        }
        if let Err(e) = self.write_language_keys(ws) {
            log::error!("write language failed: {}", e);
        }
        self.write_language_tables(ws);

        Ok(())
    }
    fn log_binary(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.unity_binary_path(&ws.config.root, name)?;
        log::info!("写入二进制: {}\n{}", self.unity_bin_relative(&name), Url::from_file_path(&path)?);
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

impl UnityCodegen {
    fn write_language_keys(&self, ws: &WorkspaceManager) -> XResult<()> {
        let mut file = self.log_binary(ws, "LanguageKeys")?;
        let table = ws.get_language_keys();
        (table.len() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
        for key in table {
            XCellValue::String(key.to_string()).write_to(&mut file, ByteOrder::LittleEndian)?
        }
        Ok(())
    }
    fn write_language_tables(&self, ws: &WorkspaceManager) {
        for language in ws.languages() {
            if let Err(e) = self.write_language_table(ws, &language) {
                log::error!("write language table {} failed: {}", language.key, e);
            }
        }
    }
    fn write_language_table(&self, ws: &WorkspaceManager, table: &XLanguageData) -> XResult<()> {
        let mut file = self.log_binary(ws, &format!("Language{}", table.key))?;
        (table.localizations.len() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
        for (key, value) in &table.localizations {
            XCellValue::String(key.to_string()).write_to(&mut file, ByteOrder::LittleEndian)?;
            XCellValue::String(value.to_string()).write_to(&mut file, ByteOrder::LittleEndian)?;
        }
        Ok(())
    }
}
