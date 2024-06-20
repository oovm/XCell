use std::ops::{AddAssign, Sub};

use xcell_provider::TableReader as XCellTableReader;
use xcell_types::{
    IntegerDescription,
    for_3rd::{BigInt, Zero},
};

use crate::{
    utils::first_not_nil,
    x_table::{
        dictionary::data::XDataLine,
        table::{ArcTableReader, TableReader},
    },
};

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
    table: ArcTableReader,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DefineManager {
    pub(crate) define: BTreeMap<String, EnumerateDescription>,
    pub(crate) enumerate: BTreeMap<String, XEnumerateData>,
    pub(crate) dict: BTreeMap<String, XDictData>,
    pub(crate) list: BTreeMap<String, XListData>,
    pub(crate) class: BTreeMap<String, XClassData>,
}

impl XEnumerateTable {
    fn new(table: ArcTableReader) -> Self {
        Self { id_column: 0, id_type: Default::default(), doc_column: 0, headers: vec![], table }
    }
    pub fn confirm(table: ArcTableReader) -> XResult<Self> {
        let fst = table.get_header(0);
        if !crate::x_table::table::TableReader::is_enumerate(&table, &fst.field_name) {
            return Err(XError::runtime_error("首格字段不是 enum"));
        }
        Ok(Self::force_confirm(table))
    }
    pub(crate) fn force_confirm(table: ArcTableReader) -> Self {
        let mut out = Self::new(table.clone());
        for header in table.headers().skip(1) {
            // skip first column
            if crate::x_table::table::TableReader::is_numeric_key(&table, &header.field_name) {
                if let Some(s) = header.typing.as_integer() {
                    out.id_column = header.column;
                    out.id_type = s.clone();
                }
                continue;
            }
            if crate::x_table::table::TableReader::is_document(&table, &header.field_name) {
                out.doc_column = header.column;
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
            if !first_not_nil(&data) {
                // 首行是空的, 数据无效且不报错
                continue;
            }
            let key = match data.get(0) {
                Some(Data::String(s)) => s.to_string(),
                Some(s) => {
                    errors.push(XError::runtime_error(format!("枚举首格字段不是字符串, 实际 {}", s)).with_y(row));
                    continue;
                }
                // 已判空
                None => unreachable!(),
            };
            let value = self.read_id(&data, &mut available_id);
            let comment = XDocument::read_document(&data, self.doc_column);
            let mut line_items = vec![];
            for header in &self.headers {
                let cell = data.get(header.column).unwrap_or(&Data::Empty);
                match header.typing.parse_cell(cell) {
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
        XDocument::default()
    }
    fn read_id(&self, row: &[Data], default_id: &mut BigInt) -> BigInt {
        match self.try_read_id(row) {
            Some(s) => s,
            None => {
                default_id.add_assign(1);
                default_id.clone().sub(1)
            }
        }
    }
    fn try_read_id(&self, row: &[Data]) -> Option<BigInt> {
        if self.id_column == 0 {
            return None;
        }
        let id = row.get(self.id_column)?;

        match self.id_type.parse_value(id) {
            Ok(o) => Some(o),
            Err(e) => {
                tracing::error!("枚举表 {} 的 id 列 {} 无法解析为整数, 错误: {}", self.enumerate_name(), self.id_column, e);
                None
            }
        }
    }
}
