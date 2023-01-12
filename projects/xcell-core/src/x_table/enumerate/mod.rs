use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct XEnumerateTable {
    /// 0 表示未设置
    pub id_column: usize,
    pub id_type: IntegerKind,
    pub comment_column: usize,
    pub headers: Vec<XCellHeader>,
    pub data: BTreeMap<String, XDataItem>,
}

impl XEnumerateTable {
    pub fn confirm(table: &CalamineTable) -> XResult<Self> {
        
    }
    
    
    pub fn read_table_data(&mut self, table: &CalamineTable, path: &Path) {
        // 防止双重 borrow
        let typing = self.headers.iter().cloned().collect_vec();
        for (x, row_raw) in table.rows().enumerate().skip(3) {
            if !first_not_nil(row_raw) {
                continue;
            }
            let mut item = XDataItem { id: Default::default(), name: "".to_string(), comment: "".to_string(), data: vec![] };
            self.read_comment_details(row_raw, &mut item);
            self.read_name(row_raw, &mut item);
            self.read_id(row_raw, &mut item);
            for typed in typing.iter() {
                let cell = match typed.parse_cell(row_raw) {
                    Ok(o) => o,
                    Err(e) => {
                        log::error!("{}", e.with_xy(x, typed.column).with_path(path));
                        Default::default()
                    }
                };
                item.data.push(cell)
            }
            self.insert(item)
        }
    }
    fn read_comment_details(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        match self.comment_column {
            0 => {}
            s => item.comment = row.get(s)?.to_string(),
        }
        None
    }
    fn read_name(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        item.name = row.get(0)?.to_string();
        None
    }
    fn read_id(&self, row: &[DataType], item: &mut XDataItem) -> Option<()> {
        match self.id_column {
            0 => {}
            s => item.id = BigInt::from_str(&row.get(s)?.to_string()).ok()?,
        }
        None
    }
    pub fn insert(&mut self, item: XDataItem) {
        self.data.insert(item.name.clone(), item);
    }
}


impl WorkspaceManager {
    pub fn link_enumerate(&mut self) {
        for table in self.file_mapping.values_mut() {
            if table.enumeration_linked {
                return;
            }
            for e in table.link_enumerate(&self.enum_mapping) {
                log::error!("{e}")
            }
        }
    }
}

impl XTable {
    pub fn link_enumerate(&mut self, all: &BTreeMap<String, EnumerateDescription>) -> Vec<XError> {
        let mut errors = vec![];
        match &mut self.data {
            XTableKind::Array(v) => link_enumerate_head(&mut v.headers, &mut errors, all),
            XTableKind::Enumerate(v) => link_enumerate_head(&mut v.headers, &mut errors, all),
            XTableKind::Class(_) => {}
            XTableKind::Dictionary(_) => {}
            XTableKind::Language(_) => {}
        };
        self.enumeration_linked = true;
        errors
    }
}

impl XCellHeader {
    pub fn link_enumerate(&mut self, all: &BTreeMap<String, EnumerateDescription>) -> XResult<()> {
        let ed = match self.typing.mut_enumerate() {
            Some(s) => s,
            None => return Ok(()),
        };
        match all.get(&ed.typing) {
            Some(v) => {
                *ed = v.clone();
                Ok(())
            }
            None => Err(XError::runtime_error(format!("未知的枚举类 `{}`", &ed.typing)).with_x(self.column)),
        }
    }
}

impl XTableKind {
    pub fn link_enumerate(&self, path: &Path) -> Validation<XTableKind> {
        let mut value = self.clone();
        let mut diagnostics = vec![];
        match &mut value {
            XTableKind::Array(v) => {
                for item in v.data.iter_mut() {
                    link_enumerate_data_line(item, &v.headers, &mut diagnostics, path)
                }
            }
            XTableKind::Enumerate(v) => {
                for (_, item) in v.data.iter_mut() {
                    link_enumerate_data_line(item, &v.headers, &mut diagnostics, path)
                }
            }
            XTableKind::Class(v) => v.link_enumerate(),
            XTableKind::Dictionary(_) => {
                todo!()
            }
            XTableKind::Language(_) => {   todo!()}
        }
        Success { value, diagnostics }
    }
}

fn link_enumerate_head(headers: &mut Vec<XCellHeader>, errors: &mut Vec<XError>, all: &BTreeMap<String, EnumerateDescription>) {
    for header in headers {
        if let Err(e) = header.link_enumerate(all) {
            errors.push(e)
        }
    }
}

fn link_enumerate_data_line(item: &mut XDataItem, headers: &[XCellHeader], errors: &mut Vec<XError>, file: &Path) {
    for (i, datum) in item.data.iter_mut().enumerate() {
        if let Err(e) = link_enumerate_data_cell(headers, i, datum) {
            errors.push(e.with_path(file))
        }
    }
}

fn link_enumerate_data_cell(headers: &[XCellHeader], index: usize, data: &mut XCellValue) -> XResult<()> {
    match headers.get(index) {
        Some(s) => data.link_enumerate(&s.typing),
        None => Err(XError::table_error("not found")),
    }
}
