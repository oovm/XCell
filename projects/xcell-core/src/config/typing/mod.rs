use xcell_types::StringDescription;
use super::*;

mod der;
mod ser;

#[derive(Debug, Clone, Default, Serialize)]
pub struct TypeMetaInfo {
    pub boolean: BooleanDescription,
    pub string: StringDescription,
}
