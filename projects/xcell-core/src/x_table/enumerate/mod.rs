use std::ops::{AddAssign, Sub};

use log::log;

use xcell_errors::for_3rd::Zero;
use xcell_types::IntegerDescription;

use super::*;

pub mod data;
mod linker;
pub mod manager;

#[derive(Clone, Debug)]
pub struct XEnumerateTable {
    /// 0 表示未设置
    id_column: usize,
    /// id 的类型
    id_type: IntegerDescription,
    /// 0 表示未设置
    doc_column: usize,
    headers: Vec<XCellHeader>,
    table: CalamineTable,
}

impl XEnumerateTable {
    fn new(table: CalamineTable) -> Self {
        Self { id_column: 0, id_type: Default::default(), doc_column: 0, headers: vec![], table }
    }
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        let fst = table.get_header(0);
        if !table.is_enumerate(&fst.field_name) {
            return Err(XError::runtime_error("首格字段不是 enum"));
        }
        Ok(Self::force_confirm(table))
    }
    pub(crate) fn force_confirm(table: &CalamineTable) -> Self {
        let mut out = Self::new(table.clone());
        for header in table.headers().skip(1) {
            // skip first column
            if table.is_document(&header.field_name) {
                out.doc_column = header.column;
            }
            if table.is_enumerate_id(&header.field_name) {
                match header.typing.as_integer() {
                    Some(s) => {
                        out.id_column = header.column;
                        out.id_type = s.clone()
                    }
                    None => continue,
                }
            }
            if !header.complete {
                continue;
            }
            out.headers.push(header.clone());
        }
        out
    }

    pub fn perform(&self, ws: &mut WorkspaceManager) -> XResult<XExportData> {
        let mut mapping = BTreeMap::default();
        let mut available_id = BigInt::zero();
        let mut data = vec![];
        for (row, data) in self.table.rows() {
            let key = match data.get(0).and_then(|s| s.get_string()) {
                Some(s) => s.to_string(),
                None => {
                    log::error!("{} 行首格不是字符串, 已跳过", row);
                    continue;
                }
            };
            let value = self.read_id(data, &mut available_id);
            let comment = XDocument::read_document(data, self.doc_column);
            for header in self.headers {
                match data.get(header.column) {
                    None => {}
                    Some(s) => {}
                }
            }
            XDataItem { id: Default::default(), name: "".to_string(), comment, data: vec![] };
            mapping.insert(key, value);
        }
        ws.enumerates.insert(EnumerateDescription {
            integer: self.id_type.kind,
            name: self.enumerate_name(),
            default: "".to_string(),
            mapping,
        })?;
        Ok(XExportData::Enumerate(box XEnumerateData { name: key.clone(), comment: self.enumerate_document(), data }))
    }

    pub fn enumerate_name(&self) -> String {
        self.table.get_name()
    }
    pub fn enumerate_document(&self) -> XDocument {
        self.table.get_header(0).comment
    }
    fn read_name(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        item.name = row.get(0)?.to_string();
        None
    }
    fn read_id(&self, row: &[DataType], default_id: &mut BigInt) -> BigInt {
        match self.try_read_id(row) {
            Some(s) => s,
            None => {
                default_id.add_assign(1);
                default_id.sub(1)
            }
        }
    }
    fn try_read_id(&self, row: &[DataType]) -> Option<BigInt> {
        if self.id_column == 0 {
            return None;
        }
        let id = row.get(self.id_column)?;
        self.id_type.parse_value(id).ok()
    }
}
