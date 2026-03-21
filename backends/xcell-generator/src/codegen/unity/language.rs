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
        writeln!(file, "// Unity generated file")?;
        writeln!(file, "")?;
        writeln!(file, "namespace {}", unity.loader.namespace)?;
        writeln!(file, "{{")?;
        writeln!(file, "    public class LanguageTable")?;
        writeln!(file, "    {{")?;
        writeln!(file, "        // Language Table generated from XCell")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}" )?;
        Ok(())
    }
    // 暂时移除 make_languages 方法，因为它依赖于不存在的字段和方法
    // fn make_languages(&self, ws: &WorkspaceManager) -> UnityLanguage {
    //     UnityLanguage {
    //         compiler_version: env!("CARGO_PKG_VERSION"),
    //         binary_path: self.storage.binary.output.clone(),
    //         config: self.clone(),
    //         language_fields: ws
    //             .languages()
    //             .iter()
    //             .map(|data| LanguageField {
    //                 class_name: data.key.to_case(Case::Pascal),
    //                 public_name: data.key.to_case(Case::Camel),
    //                 private_name: format!("_{}", data.key.to_case(Case::Snake)),
    //             })
    //             .collect(),
    //     }
    // }
}
