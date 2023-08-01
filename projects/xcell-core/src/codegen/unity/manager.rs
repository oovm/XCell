use super::*;

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
        let out = match self.make_manager(ws).render() {
            Ok(o) => o,
            Err(e) => Err(XError::runtime_error(format!("生成管理器失败: {}", e)))?,
        };
        let mut file = self.log_csharp(ws, &ws.config.unity.manager_name.to_string())?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    fn make_manager(&self, ws: &WorkspaceManager) -> UnityManager {
        UnityManager {
            compiler_version: env!("CARGO_PKG_VERSION"),
            data_version: ws.config.version.clone(),
            edit_time: XCellValue::csharp_now(),
            config: ws.config.unity.clone(),
            tables: ws
                .class_names()
                .iter()
                .map(|name| {
                    let name = format!("{name}{}", self.suffix_table);
                    TableField {
                        typing: name.to_case(Case::Pascal),
                        public_name: name.to_case(Case::Camel),
                        private_name: format!("_{}", name.to_case(Case::Snake)),
                    }
                })
                .collect(),
        }
    }
}
