use super::*;
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use xcell_analyzer::{UnityCodegen, WorkspaceManager};
use xcell_types::{XError, XResult};

#[derive(Template)]
#[template(path = "BuildLanguage.cs", ext = "txt", escape = "none")]
pub struct UnityLanguage {
    compiler_version: &'static str,
    binary_path: String,
    config: UnityCodegen,
    language_fields: Vec<LanguageField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LanguageField {
    class_name: String,
    public_name: String,
    private_name: String,
}

impl Display for LanguageField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub(super) fn write_language(&self, ws: &WorkspaceManager) -> XResult<()> {
        let unity = &ws.config.unity;
        let root = &ws.config.root;
        
        let output_dir = PathBuf::from(&unity.loader.output);
        let output_dir = match output_dir.is_absolute() {
            true => output_dir,
            false => root.join(output_dir),
        };
        
        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let path = output_dir.join("LanguageTable.cs");
        let mut file = std::fs::File::create(path)?;
        let out = self.make_languages(ws).render()?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    /// Creates Unity language template data
    ///
    /// # Arguments
    /// * `ws` - Workspace manager
    ///
    /// # Returns
    /// Unity language template data
    fn make_languages(&self, ws: &WorkspaceManager) -> UnityLanguage {
        UnityLanguage {
            compiler_version: env!("CARGO_PKG_VERSION"),
            binary_path: "".to_string(),
            config: self.clone(),
            language_fields: Vec::new(),
        }
    }
}
