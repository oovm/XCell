use super::*;
use xcell_types::{ExtraTypes, StringDescription};

mod der;
mod ser;

#[derive(Debug, Clone, Default, Serialize)]
pub struct TypeMetaInfo {
    pub boolean: BooleanDescription,
    pub string: StringDescription,
    pub extra: ExtraTypes,
}
