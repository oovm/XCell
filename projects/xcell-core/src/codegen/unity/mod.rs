use super::*;

mod binary;
mod dictionary;
mod enumerate;
mod manager;

impl UnityCodegen {
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if let Some(s) = self.unity_xml_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        Ok(())
    }
    pub fn write_csharp(&self, ws: &WorkspaceManager) -> XResult<()> {
        if let Some(s) = self.unity_csharp_path(&ws.config.root, "test")?.parent() {
            create_dir_all(s)?
        }
        for table in ws.enumerates() {
            if let Err(e) = self.write_enumerate(ws, table) {
                log::error!("生成枚举失败: {}", e);
            }
        }
        for table in ws.dicts() {
            if let Err(e) = self.write_dict(ws, table) {
                log::error!("生成枚举失败: {}", e);
            }
        }

        self.write_manager(ws)?;
        Ok(())
    }
    fn log_csharp(&self, ws: &WorkspaceManager, name: &str) -> XResult<File> {
        let path = self.unity_csharp_path(&ws.config.root, name)?;
        log::info!("写入 C#: {}\n{}", self.unity_cs_relative(name), Url::from_file_path(&path)?);
        Ok(File::create(path)?)
    }
}

