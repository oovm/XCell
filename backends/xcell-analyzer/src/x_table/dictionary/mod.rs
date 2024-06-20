use xcell_provider::TableReader as XCellTableReader;
use xcell_types::IntegerDescription;

use crate::{
    utils::first_not_nil,
    x_table::{
        dictionary::data::XDataLine,
        table::{ArcTableReader, TableReader},
    },
};

use super::*;

pub mod data;
pub mod manager;

#[derive(Clone, Debug)]
pub struct XListTable {
    table: ArcTableReader,
    id_type: IntegerDescription,
    headers: Vec<XCellHeader>,
}

#[derive(Clone, Debug)]
pub struct XDictTable {
    table: ArcTableReader,
    headers: Vec<XCellHeader>,
}

impl XListTable {
    pub fn confirm(table: ArcTableReader) -> XResult<Self> {
        if !crate::x_table::table::TableReader::is_list(&table) {
            return Err(XError::runtime_error("首格字段不是 id"));
        }
        let head = table.get_header(0);
        let mut out = match head.typing.as_integer() {
            Some(s) => Self { table, id_type: s.clone(), headers: vec![] },
            None => return Err(XError::runtime_error("首格字段类型不是整数")),
        };
        // TODO: 实现 headers() 方法
        // for header in table.headers() {
        //     if header.complete {
        //         out.headers.push(header);
        //     }
        // }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut errors = vec![];
        let mut values = BTreeMap::default();
        // TODO: 实现 rows() 方法
        // for (row, data) in self.table.rows() {
        //     if !first_not_nil(data) {
        //         // 首行是空的, 数据无效且不报错
        //         continue;
        //     }
        //     match XDataLine::parse_id_cell(data, row, &self.headers, &mut errors) {
        //         Ok(o) => {
        //             values.insert(o.id.clone(), o);
        //         }
        //         Err(e) => errors.push(e.with_y(row)),
        //     }
        // }
        ws.add_list(XListData {
            name: self.table.get_name(),
            id_type: self.id_type.kind,
            headers: self.headers.clone(),
            mapping: values,
        });
        errors
    }
}

impl XDictTable {
    pub fn confirm(table: ArcTableReader) -> XResult<Self> {
        if !crate::x_table::table::TableReader::is_dict(&table) {
            return Err(XError::runtime_error("首格字段不是 key"));
        }
        let mut out = Self { table, headers: vec![] };
        // TODO: 实现 headers() 方法
        // for header in table.headers() {
        //     if header.complete {
        //         out.headers.push(header);
        //     }
        // }
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut errors = vec![];
        let mut values = BTreeMap::default();
        // TODO: 实现 rows() 方法
        // for (row, data) in self.table.rows() {
        //     match XDataLine::parse_key_cell(data, row, &self.headers, &mut errors) {
        //         Ok(o) => {
        //             values.insert(o.key.clone(), o);
        //         }
        //         Err(e) => {
        //             errors.push(e.with_y(row));
        //         }
        //     }
        // }
        ws.add_dict(XDictData { name: self.table.get_name(), headers: self.headers.clone(), mapping: values });
        errors
    }
}
