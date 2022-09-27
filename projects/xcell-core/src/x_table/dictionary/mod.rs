use super::*;

pub mod data;
pub mod manager;

#[derive(Clone, Debug)]
pub struct XListTable {
    table: CalamineTable,
    headers: Vec<XCellHeader>,
}

#[derive(Clone, Debug)]
pub struct XDictTable {
    table: CalamineTable,
    headers: Vec<XCellHeader>,
}

impl XListTable {
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
        let mut errors = vec![];
        let mut values = BTreeMap::default();
        for (row, data) in self.table.rows().skip(1) {
            match XDataItem::parse_id_cell(data, &mut errors) {
                Ok(o) => {
                    values.insert(o.id.clone(), o);
                }
                Err(e) => {
                    log::error!("{}", e.with_y(row));
                }
            }
        }
        Ok(XExportData::List(box XListData { map: values }))
    }
}

impl XDictTable {
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        let header = table.get_header(0);
        if !table.is_array(&header.field_name) {
            return Err(XError::runtime_error("首格字段不是 key"));
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
        let mut errors = vec![];
        let mut values = BTreeMap::default();
        for (row, data) in self.table.rows().skip(1) {
            match XDataItem::parse_key_cell(data, &mut errors) {
                Ok(o) => {
                    values.insert(o.key.clone(), o);
                }
                Err(e) => {
                    log::error!("{}", e.with_y(row));
                }
            }
        }
        Ok(XExportData::Dict(box XDictData { map: values }))
    }
}
