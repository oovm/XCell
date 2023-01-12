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
pub struct XListData {
    map: BTreeMap<BigInt, XDataItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDictData {
    map: BTreeMap<String, XDataItem>,
}

/// 表单中的一行数据
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XDataItem {
    /// 该表单数据的编号
    pub id: BigInt,
    /// 该表单数据的键
    pub key: String,
    /// 该表单数据的注释
    pub comment: XDocument,
    /// 该表单数据的有效值
    pub data: Vec<XCellValue>,
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
        let mut errors = vec![];
        let mut values = BTreeMap::default();
        for (row, data) in self.table.rows().skip(1) {
            match XDataItem::parse_id_cell(data, &mut errors) {
                Ok(o) => {
                    values.push(o);
                }
                Err(e) => {
                    log::error!("{}", e.with_y(row));
                }
            }
        }
        Ok(XExportData::List(box XListData { map: values }))
    }
}

impl XDictData {
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
                    values.push(o);
                }
                Err(e) => {
                    log::error!("{}", e.with_y(row));
                }
            }
        }
        Ok(XExportData::List(box XListData { map: values }))
    }
}
