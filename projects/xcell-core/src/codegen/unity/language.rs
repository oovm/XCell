use super::*;

#[derive(Template)]
#[template(path = "BuildLanguage.cs.djv", ext = "txt", escape = "none")]
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
        let out = match self.make_languages(ws).render() {
            Ok(o) => o,
            Err(e) => Err(XError::runtime_error(format!("生成语言表失败: {e}")))?,
        };
        let mut file = self.log_csharp(ws, "LanguageTable")?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    fn make_languages(&self, ws: &WorkspaceManager) -> UnityLanguage {
        UnityLanguage {
            compiler_version: env!("CARGO_PKG_VERSION"),
            binary_path: self.binary.output.clone(),
            config: self.clone(),
            language_fields: ws
                .languages()
                .iter()
                .map(|data| LanguageField {
                    class_name: data.key.to_case(Case::Pascal),
                    public_name: data.key.to_case(Case::Camel),
                    private_name: format!("_{}", data.key.to_case(Case::Snake)),
                })
                .collect(),
        }
    }
}
