use xcell_types::XCellValue;

use crate::XDataItem;

use super::*;

pub struct DataContractWriter {}

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
    pub fn write_xml(&self, table: &XCellTable, output: &Path) -> XResult<()> {
        let mut ctx = Context::new();
        ctx.insert("items", &table.as_xml());
        tera_render(include_str!("DataContract.xml"), &ctx, &output, "DataContract.xml")?;
        Ok(())
    }
}

impl XCellTable {
    fn as_xml(&self) -> Vec<XmlItem> {
        self.data.rows().iter().map(|v| XmlItem { fields: v.as_xml() }).collect()
    }
}

impl XDataItem {
    fn as_xml(&self) -> Vec<XmlField> {
        let mut out = vec![];
        for datum in &self.data {
            let data = match datum {
                XCellValue::Boolean(v) => v.to_string(),
                _=> datum.to_string()
            };
            out.push(XmlField {
                name: self.name.to_string(),
                data,
            })
        }
        out
    }
}
