use itertools::Itertools;

use crate::{utils::first_not_nil, CalamineTable3};

use super::*;

impl XArrayTable {
    pub fn read_table_data(&mut self, table: &CalamineTable3, path: &Path) {
        // 防止双重 borrow
        let typing = self.headers.iter().cloned().collect_vec();
        let rows = table.rows().skip(3).filter(|v| first_not_nil(v));
        for (x, row_raw) in rows.enumerate() {
            let mut item = XDataItem { id: Default::default(), name: "".to_string(), comment: "".to_string(), data: vec![] };
            for (y, typed) in typing.iter().enumerate() {
                let cell = match typed.parse_cell(row_raw) {
                    Ok(o) => o,
                    Err(e) => {
                        log::error!("{}", e.with_xy(x, y).with_path(path));
                        Default::default()
                    }
                };
                item.data.push(cell)
            }
            self.insert(item)
        }
    }
    pub fn insert(&mut self, item: XDataItem) {
        self.data.push(item);
    }
}
