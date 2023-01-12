use itertools::Itertools;

use crate::{utils::first_not_nil, x_table::export::XDataItem};

use super::*;

pub mod data;

#[derive(Clone, Debug)]
pub struct XArrayTable {
    table: CalamineTable,
    headers: Vec<XCellHeader>,
}

#[derive(Clone, Debug)]
pub struct XDictionaryTable {
    table: CalamineTable,
    headers: Vec<XCellHeader>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XArrayData {
    pub values: BTreeMap<BigInt, XDataItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDictionaryData {
    pub values: BTreeMap<String, XDataItem>,
}

impl XArrayTable {
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        let header = table.get_header(0);
        if !table.is_array(&header.field_name) {
            return Err(XError::runtime_error("首格字段不是 id"));
        }
        let mut out = Self { table: table.clone(), headers: vec![] };
        for header in table.headers() {
            if header.complete {
                out.headers.push(header);
            }
        }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<XExportData> {
        let mut items = vec![];
        for (row, data) in self.table.rows().skip(1) {
            match XArrayData::parse_cell() {
                Ok(o) => {
                    items.push(o);
                }
                Err(e) => {
                    log::error!("{}", e.with_y(row));
                }
            }
        }
        Ok(XExportData::Array(box XArrayData { name: self.table.get_name(), items }))
    }
}

impl XArrayData {
    pub fn parse_cell(&self, data: &[DataType]) -> XResult<Self> {}
    fn try_parse_key(&self, data: &[DataType]) -> XResult<BigInt> {
        match data.get(0) {
            Some(_) => {}
            None => Err(XError::runtime_error("id 不能为空"))?,
        }
    }
}

impl XArrayTable {
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path) {
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
