use std::ops::{AddAssign, Sub};

use log::log;

use xcell_errors::for_3rd::Zero;
use xcell_types::IntegerDescription;

use crate::{CalamineTable, XDataItem};

use super::*;

mod linker;
mod manager;

#[derive(Clone, Debug)]
pub struct XEnumerateTable {
    /// 0 表示未设置
    id_column: usize,
    id_type: IntegerDescription,
    doc_column: usize,
    headers: Vec<XCellHeader>,
    table: CalamineTable,
}

impl XEnumerateTable {
    fn new(table: CalamineTable) -> Self {
        Self { id_column: 0, id_type: Default::default(), doc_column: 0, headers: vec![], table }
    }
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        if table.is_enumerate() {
            return Err(XError::runtime_error("首格字段不是 enum"));
        }
        let mut out = Self::new(table.clone());
        for header in table.headers().skip(1) {
            // skip first column
            if table.is_document(&header.field_name) {
                out.doc_column = header.column;
            }
            if table.is_enumerate_id(&header.field_name) {
                match header.typing.as_integer() {
                    Some(s) => out.id_type = s.clone(),
                    None => continue,
                }
            }
            if !header.complete {
                continue;
            }
            out.headers.push(header.clone());
        }
        Ok(out)
    }

    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<()> {
        let mut mapping = BTreeMap::default();
        let mut available_id = BigInt::zero();
        for (row, data) in self.table.rows() {
            let key = match data.get(0).and_then(|s| s.get_string()) {
                Some(s) => s.to_string(),
                None => {
                    log::error!("{} 行首格不是字符串", row);
                    continue;
                }
            };
            let value = self.read_id(data, &mut available_id);

            match data.get(self.doc_column) {
                None => {}
                Some(_) => {}
            }
            XDataItem { id: Default::default(), name: "".to_string(), comment: "".to_string(), data: vec![] };

            mapping.insert(key, value);
        }
        EnumerateDescription { integer: Default::default(), typing: "".to_string(), default: "".to_string(), mapping };
        Ok(())
    }
    fn read_comment_details(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        match self.doc_column {
            0 => {}
            s => item.comment = row.get(s)?.to_string(),
        }
        None
    }
    fn read_name(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        item.name = row.get(0)?.to_string();
        None
    }

    fn read_id(&self, row: &[DataType], default_id: &mut BigInt) -> BigInt {
        let id: Option<BigInt> = try {
            let id = row.get(self.id_column)?;
            self.id_type.parse_value(id).ok()?
        };
        match id {
            Some(s) => s,
            None => {
                default_id.add_assign(1);
                default_id.sub(1)
            }
        }
    }
    pub fn insert(&mut self, item: XDataItem) {
        self.data.insert(item.name.clone(), item);
    }
}
