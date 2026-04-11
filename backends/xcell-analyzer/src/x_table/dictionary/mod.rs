use xcell_provider::TableReader as XCellTableReader;
use xcell_core::{ByteOrder, IntegerDescription, IntegerKind};

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
        // 总是返回错误，这样所有的表都会被尝试解析为其他类型的表，最后被解析为 dict 表
        Err(XError::runtime_error("所有表都被视为 dict 表"))
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut errors = vec![];
        let mut values = BTreeMap::default();
        // 实现 rows() 方法
        for (row, data) in self.table.rows() {
            if !first_not_nil(&data) {
                // 首行是空的, 数据无效且不报错
                continue;
            }
            match XDataLine::parse_id_cell(&data, row, &self.headers, &mut errors) {
                Ok(o) => {
                    values.insert(o.id.clone(), o);
                }
                Err(e) => errors.push(e.with_y(row)),
            }
        }
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
        // 根据 dict.md 文档，默认即为 Dict 类型，无需显式标记
        // 所有表都被视为 dict 表
        // 先获取 headers
        let headers: Vec<XCellHeader> = table.headers().collect();
        let mut out = Self { table, headers };
        // 实现 headers() 方法
        Ok(out)
    }
    pub fn perform(&self, ws: &mut WorkspaceManager) -> Vec<XError> {
        let mut errors = vec![];
        let mut values = BTreeMap::default();
        // 实现 rows() 方法
        for (row, data) in self.table.rows() {
            if !crate::utils::first_not_nil(&data) {
                // 首行是空的, 数据无效且不报错
                continue;
            }
            match XDataLine::parse_key_cell(&data, row, &self.headers, &mut errors) {
                Ok(o) => {
                    values.insert(o.key.clone(), o);
                }
                Err(e) => {
                    errors.push(e.with_y(row));
                }
            }
        }
        ws.add_dict(XDictData { name: self.table.get_name(), headers: self.headers.clone(), mapping: values });
        errors
    }
}
