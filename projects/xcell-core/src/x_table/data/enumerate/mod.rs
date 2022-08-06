use itertools::Itertools;

use crate::{utils::first_not_nil, CalamineTable};

use super::*;

impl XData {
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path) {
        let res = match self {
            XData::Dictionary(_) => {
                todo!()
            }
            XData::Enumerate(v) => v.read_table_data(table, path),
        };
        match res {
            Ok(_) => {}
            Err(e) => {
                log::error!("{}", e.with_path(path))
            }
        }
    }
}

impl XDataEnumerate {
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path) -> XResult<()> {
        let rows = table.rows().skip(3).filter(|v| first_not_nil(v));
        // 防止双重 borrow
        let typing = self.headers.iter().cloned().collect_vec();
        let dict = &mut self.data;
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
        Ok(())
    }
    pub fn insert(&mut self, item: XDataItem) {}
}
