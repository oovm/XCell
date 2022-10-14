use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct XDocument {
    pub summary: String,
    pub detail: String,
}

impl From<&DataType> for XDocument {
    fn from(value: &DataType) -> Self {
        let mut out = XDocument::default();
        if let Some(s) = value.get_string() {
            out.summary = s.to_string()
        }
        out
    }
}

impl XDocument {
    pub fn read_document(row: &[DataType], id: usize) -> Self {
        row.get(id).map(XDocument::from).unwrap_or_default()
    }
    pub fn read_non_zero(row: &[DataType], id: usize) -> Self {
        if id == 0 {
            return XDocument::default();
        }
        row.get(id).map(XDocument::from).unwrap_or_default()
    }
    pub fn lines(&self) -> Vec<String> {
        let mut out = String::new();
        if !self.summary.trim().is_empty() {
            out.push_str("<summary>");
            out.push('\n');
            out.push_str(&self.summary);
            out.push('\n');
            out.push_str("</summary>");
            out.push('\n');
        }
        if !self.detail.trim().is_empty() {
            out.push_str("<detail>");
            out.push('\n');
            out.push_str(&self.detail);
            out.push('\n');
            out.push_str("</detail>");
        }
        out.lines().map(|s| s.to_string()).collect()
    }
}
