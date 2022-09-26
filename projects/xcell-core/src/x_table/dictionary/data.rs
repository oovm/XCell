use std::collections::btree_map::Values;

use super::*;

impl XDataItem {
    pub fn parse_key_cell(data: &[DataType], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.key = out.try_parse_key(data)?;

        for (column, datum) in data.iter().enumerate().skip(1) {}
    }

    fn try_parse_key(&self, data: &[DataType]) -> XResult<String> {
        match data.get(0) {
            Some(s) => s.get_string(),
            None => Err(XError::runtime_error("id 不能为空"))?,
        }
    }
}

impl XDataItem {
    pub fn parse_id_cell(data: &[DataType], errors: &mut Vec<XError>) -> XResult<Self> {
        let mut out = Self::default();
        out.key = out.try_parse_key(data)?;

        for (column, datum) in data.iter().enumerate().skip(1) {}
    }

    fn try_parse_id(&self, data: &[DataType]) -> XResult<String> {
        match data.get(0) {
            Some(s) => s.get_string(),
            None => Err(XError::runtime_error("id 不能为空"))?,
        }
    }
}

impl XListData {
    pub fn length(&self) -> usize {
        self.map.len()
    }
    pub fn values(&self) -> Values<'_, BigInt, XDataItem> {
        self.map.values()
    }
}

impl XDictData {
    pub fn length(&self) -> usize {
        self.map.len()
    }
    pub fn values(&self) -> Values<'_, String, XDataItem> {
        self.map.values()
    }
}
