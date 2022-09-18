use crate::x_table::table::CalamineTable2;

use super::*;

pub struct LanguageManager {}

impl Default for LanguageManager {
    fn default() -> Self {
        Self {}
    }
}

pub struct LanguageItem {
    group: String,
    key: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XLanguageTable {
    value_column: usize,
    group_column: usize,
}


impl XLanguageTable {
    pub fn perform(ws: &mut WorkspaceManager, table: &CalamineTable2) -> XResult<()> {
        if table.get_header(0).typing != XCellTyped::LanguageKey {
            return Err(XError::runtime_error("首格类型不是 LanguageKey"));
        }
        let mut out = XLanguageTable {
            value_column: 0,
            group_column: 0,
        };
        for header in table.headers() {
            if header.typing == XCellTyped::LanguageValue {
                out.value_column = header.column;
            }
            if header.field_name.eq_ignore_ascii_case("group") || header.field_name.eq_ignore_ascii_case("namespace") {
                out.group_column = header.column;
            }
        }


        ws.lang_mapping;
    }
}