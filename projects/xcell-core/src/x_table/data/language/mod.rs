use crate::config::TableLineMode;

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XLanguageTable {}



impl XLanguageTable {
    pub fn perform(ws: &mut WorkspaceManager, table: &CalamineTable2) -> XResult<()> {
        ws.lang_mapping;
    }
}