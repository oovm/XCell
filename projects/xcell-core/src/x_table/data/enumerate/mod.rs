use std::str::FromStr;

use itertools::Itertools;

use xcell_errors::Validation;
use xcell_types::EnumerateDescription;

use crate::{utils::first_not_nil, CalamineTable, Success};

use super::*;

impl XData {
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path) {
        let res = match self {
            XData::Dictionary(v) => v.read_table_data(table, path),
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
        // 防止双重 borrow
        let typing = self.headers.iter().cloned().collect_vec();
        for (x, row_raw) in table.rows().enumerate().skip(3) {
            if !first_not_nil(row_raw) {
                continue;
            }
            let mut item = XDataItem { id: Default::default(), name: "".to_string(), comment: "".to_string(), data: vec![] };
            self.read_comment_details(row_raw, &mut item);
            self.read_name(row_raw, &mut item);
            self.read_id(row_raw, &mut item);
            for typed in typing.iter() {
                let cell = match typed.parse_cell(row_raw) {
                    Ok(o) => o,
                    Err(e) => {
                        log::error!("{}", e.with_xy(x, typed.column).with_path(path));
                        Default::default()
                    }
                };
                item.data.push(cell)
            }
            self.insert(item)
        }
        Ok(())
    }
    fn read_comment_details(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        match self.comment_column {
            0 => {}
            s => item.comment = row.get(s)?.to_string(),
        }
        None
    }
    fn read_name(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        item.name = row.get(0)?.to_string();
        None
    }
    fn read_id(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        match self.id_column {
            0 => {}
            s => item.id = BigInt::from_str(&row.get(s)?.to_string()).ok()?,
        }
        None
    }
    pub fn insert(&mut self, item: XDataItem) {
        self.data.insert(item.name.clone(), item);
    }
}

impl XCellTable {
    pub fn link_enumerate(&self, all: &BTreeMap<String, EnumerateDescription>) -> Validation<XCellTable> {
        let mut diagnostics = vec![];
        let mut value = self.clone();
        match &mut value.data {
            XData::Dictionary(_) => {}
            XData::Enumerate(v) => v.link_enumerate_head(all).option(|e| diagnostics.push(e)).unwrap_or_default(),
        };
        Success { value, diagnostics }
    }
}

impl XDataEnumerate {
    pub fn link_enumerate_head(&mut self, all: &BTreeMap<String, EnumerateDescription>) -> Validation<()> {
        let mut diagnostics = vec![];
        for header in &mut self.headers {
            let ed = match header.typing.mut_enumerate() {
                Some(s) => s,
                None => continue,
            };
            match all.get(&ed.typing) {
                Some(v) => *ed = v.clone(),
                None => diagnostics.push(XError::runtime_error(format!("未知的枚举类: {}", &ed.typing)).with_x(header.column)),
            }
        }
        Success { value: (), diagnostics }
    }
}
