use super::*;
use crate::EnumerateManager;

impl WorkspaceManager {
    pub fn link_enumerate(&mut self) {
        for table in self.file_mapping.values_mut() {
            if table.enumeration_linked {
                return;
            }
            for e in table.link_enumerate(&self.enumerates) {
                log::error!("{e}")
            }
        }
    }
}

impl XTable {
    pub fn link_enumerate(&mut self, all: &EnumerateManager) -> Vec<XError> {
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
    pub fn link_enumerate(&mut self, all: &EnumerateManager) -> XResult<()> {
        let ed = match self.typing.mut_enumerate() {
            Some(s) => s,
            None => return Ok(()),
        };
        match all.get(&ed.name) {
            Some(v) => {
                *ed = v.clone();
                Ok(())
            }
            None => Err(XError::runtime_error(format!("未知的枚举类 `{}`", &ed.name)).with_x(self.column)),
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
            XTableKind::Language(_) => {
                todo!()
            }
        }
        Success { value, diagnostics }
    }
}

fn link_enumerate_head(headers: &mut Vec<XCellHeader>, errors: &mut Vec<XError>, all: &EnumerateManager) {
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
