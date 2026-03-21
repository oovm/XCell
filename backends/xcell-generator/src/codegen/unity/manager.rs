use super::*;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use serde::{Deserialize, Serialize};
use dejavu_macros::Template;
use xcell_analyzer::{UnityCodegen, WorkspaceManager};
use xcell_types::{XCellValue, XError, XResult};

#[derive(Template)]
#[template(path = "BuildManager.cs", ext = "txt", escape = "none")]
pub struct UnityManager {
    compiler_version: &'static str,
    data_version: String,
    edit_time: String,
    config: UnityCodegen,
    tables: Vec<TableField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableField {
    typing: String,
    public_name: String,
    private_name: String,
}

impl Display for TableField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub(super) fn write_manager(&self, ws: &WorkspaceManager) -> XResult<()> {
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
        
        let path = output_dir.join("DataTableManager.cs");
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "// Unity generated file")?;
        writeln!(file, "")?;
        writeln!(file, "namespace {}", unity.loader.namespace)?;
        writeln!(file, "{{")?;
        writeln!(file, "    public class DataTableManager")?;
        writeln!(file, "    {{")?;
        writeln!(file, "        // DataTable Manager generated from XCell")?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}" )?;
        Ok(())
    }
    // 暂时移除 make_manager 方法，因为它依赖于不存在的字段和方法
    // fn make_manager(&self, ws: &WorkspaceManager) -> UnityManager {
    //     UnityManager {
    //         compiler_version: env!("CARGO_PKG_VERSION"),
    //         data_version: ws.config.version.clone(),
    //         edit_time: XCellValue::csharp_now(),
    //         config: ws.config.unity.clone(),
    //         tables: ws
    //             .class_names()
    //             .iter()
    //             .map(|name| {
    //                 let name = format!("{name}{}", self.suffix_table);
    //                 TableField {
    //                     typing: name.to_case(Case::Pascal),
    //                     public_name: name.to_case(Case::Camel),
    //                     private_name: format!("_{}", name.to_case(Case::Snake)),
    //                 }
    //             })
    //             .collect(),
    //     }
    // }
}
