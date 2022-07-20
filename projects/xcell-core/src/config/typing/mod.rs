use xcell_types::BooleanDescription;
use super::*;

mod der;
mod ser;

#[derive(Debug, Clone, Default, Serialize)]
pub struct TypeMetaInfo {
    #[serde(default, alias = "bool")]
    pub boolean: BooleanDescription,
}
