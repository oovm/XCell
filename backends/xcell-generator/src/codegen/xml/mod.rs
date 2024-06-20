use super::*;
use std::path::Path;
use xcell_analyzer::{XCellHeader, XDataLine};
use xcell_types::{XCellValue, XResult};

#[derive(Clone, Debug, Default, Serialize)]
pub struct XmlCodegen {
    /// Whether to generate XML data
    pub enable: bool,
    /// Output directory
    pub output: String,
}

#[derive(Serialize)]
pub struct DataContractWriter {
    namespace: String,
    class_name: String,
    table_name: String,
    items: Vec<XmlItem>,
}

#[derive(Serialize)]
pub struct XmlItem {
    is_vector: bool,
    fields: Vec<XmlField>,
}

#[derive(Serialize)]
pub struct XmlField {
    name: String,
    data: String,
}

impl XmlCodegen {
    /// XML output directory
    pub fn xml_path(&self, root: &Path, file_name: &str) -> XResult<std::path::PathBuf> {
        let dir = root.join(&self.output);
        let path = dir.join(file_name).with_extension("xml");
        Ok(path)
    }

    /// XML relative path
    pub fn xml_relative(&self, file_name: &str) -> String {
        format!("{}/{}.xml", self.output, file_name)
    }

    /// Ensure output directories exist
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if self.enable {
            if let Some(s) = self.xml_path(root, "test")?.parent() {
                std::fs::create_dir_all(s)?;
            }
        }
        Ok(())
    }
}

impl DataContractWriter {
    pub fn write_xml(&self, output: &Path) -> XResult<()> {
        // let ctx = Context::from_serialize(self)?;
        // tera_render(include_str!("DataContract.xml.dj"), &ctx, output, "DataContract.xml")?;
        Ok(())
    }
}

impl XDataLine {
    fn as_xml(&self, headers: &[&XCellHeader]) -> Vec<XmlField> {
        let mut out = vec![];

        for (i, datum) in self.data.iter().enumerate() {
            let field = match headers.get(i) {
                Some(s) => s.field_name.to_string(),
                None => break,
            };
            let data = match datum {
                XCellValue::Boolean(v) => v.to_string(),
                _ => datum.to_string(),
            };
            out.push(XmlField { name: field, data })
        }
        out.into_iter().collect()
    }
}

impl super::Codegen for XmlCodegen {
    fn generate(&self, context: &super::CodegenContext) -> XResult<()> {
        // TODO: Implement XML code generation
        Ok(())
    }

    fn name(&self) -> &'static str {
        "xml"
    }
}
