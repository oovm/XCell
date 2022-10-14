use std::ops::{AddAssign, Sub};

use xcell_errors::for_3rd::Zero;
use xcell_types::IntegerDescription;

use crate::{utils::first_not_nil, x_table::dictionary::data::XDataLine};

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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DefineManager {
    define: BTreeMap<String, EnumerateDescription>,
    enumerate: BTreeMap<String, XEnumerateData>,
    dict: BTreeMap<String, XDictData>,
    list: BTreeMap<String, XListData>,
    class: BTreeMap<String, XClassData>,
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
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut errors = vec![];
        let mut define = EnumerateDescription::new(self.enumerate_name());
        let mut available_id = BigInt::zero();
        let mut data_items = vec![];
        for (row, data) in self.table.rows() {
            if !first_not_nil(data) {
                // 首行是空的, 数据无效且不报错
                continue;
            }
            let key = match data.get(0) {
                Some(DataType::String(s)) => s.to_string(),
                Some(s) => {
                    errors.push(XError::runtime_error(format!("枚举首格字段不是字符串, 实际 {}", s)).with_y(row));
                    continue;
                }
                // 已判空
                None => unreachable!(),
            };
            let value = self.read_id(data, &mut available_id);
            let comment = XDocument::read_document(data, self.doc_column);
            let mut line_items = vec![];
            for header in &self.headers {
                match header.parse_cell(data) {
                    Ok(o) => line_items.push(o),
                    Err(e) => {
                        errors.push(e.with_y(row));
                        line_items.push(Default::default())
                    }
                };
            }
            data_items.push(XDataLine { id: value.clone(), key: key.clone(), comment, data: line_items });
            if let Err(e) = define.add_mapping(&key, value) {
                errors.push(e.with_y(row));
                return errors;
            }
        }
        ws.add_enumerate(XEnumerateData {
            name: define.name.clone(),
            typing: self.id_type.clone(),
            comment: self.enumerate_document(),
            headers: self.headers.clone(),
            lines: data_items,
        });
        define.integer = self.id_type.kind;
        if let Err(e) = ws.add_define(define) {
            errors.push(e);
        }
        errors
    }
    pub fn enumerate_name(&self) -> String {
        self.table.get_name()
    }
    pub fn enumerate_document(&self) -> XDocument {
        self.table.get_header(0).comment
    }
    fn read_id(&self, row: &[DataType], default_id: &mut BigInt) -> BigInt {
        match self.try_read_id(row) {
            Some(s) => s,
            None => {
                default_id.add_assign(1);
                default_id.clone().sub(1)
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
