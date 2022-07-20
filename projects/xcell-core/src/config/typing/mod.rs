use super::*;

mod der;
mod ser;

#[derive(Debug, Clone, Default, Serialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    pub boolean: BooleanMetaInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct BooleanMetaInfo {
    pub accept: BTreeSet<String>,
    pub reject: BTreeSet<String>,
    pub default: bool,
}
