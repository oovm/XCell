use crate::x_table::dictionary::data::XDataLine;
use super::*;

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

impl DataContractWriter {
    pub fn write_xml(&self, output: &Path) -> XResult<()> {
        let ctx = Context::from_serialize(self)?;
        tera_render(include_str!("DataContract.xml.djv"), &ctx, output, "DataContract.xml")?;
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
        out.into_iter().sorted_by_key(|v| v.name.to_owned()).collect()
    }
}
