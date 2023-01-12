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
    pub fn lines() -> Vec<String> {
        vec![]
    }
}
