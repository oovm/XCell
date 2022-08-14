use xcell_types::XCellValue;

use crate::XDataItem;

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
    fields: Vec<XmlField>,
}

#[derive(Serialize)]
pub struct XmlField {
    name: String,
    data: String,
}

impl DataContractWriter {
    pub fn new(namespace: &str, table: &XCellTable, table_suffix: &str) -> Self {
        Self {
            class_name: table.name.clone(),
            table_name: format!("{}{}", table.name, table_suffix),
            namespace: namespace.to_string(),
            items: table.as_xml(),
        }
    }
    pub fn write_xml(&self, output: &Path) -> XResult<()> {
        let ctx = Context::from_serialize(self)?;
        tera_render(include_str!("DataContract.xml"), &ctx, &output, "DataContract.xml")?;
        Ok(())
    }
}

impl XCellTable {
    fn as_xml(&self) -> Vec<XmlItem> {
        self.data.rows().iter().map(|v| XmlItem { fields: v.as_xml(&self.name) }).collect()
    }
}

impl XDataItem {
    fn as_xml(&self, name: &str) -> Vec<XmlField> {
        let mut out = vec![];
        for datum in &self.data {
            let data = match datum {
                XCellValue::Boolean(v) => v.to_string(),
                _ => datum.to_string(),
            };
            out.push(XmlField { name: name.to_string(), data })
        }
        out
    }
}
