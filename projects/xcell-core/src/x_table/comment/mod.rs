use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct XComment {
    pub summary: String,
    pub detail: String,
}

impl From<&DataType> for XComment {
    fn from(value: &DataType) -> Self {
        let mut out = Default::default();
        if let Some(s) = value.get_string() {
            out.summary = s.to_string()
        }
        out
    }
}

impl XComment {
    pub fn read_document(row: &[DataType], id: usize) -> Self {
        row.get(id).map(XComment::from).unwrap_or_default()
    }

    pub fn lines() -> Vec<String> {
        vec![]
    }
}
