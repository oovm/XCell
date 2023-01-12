use crate::EnumerateManager;

use super::*;

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

impl XExportData {
    pub fn link_enumerate(&mut self, all: &EnumerateManager) -> Vec<XError> {
        match self {
            XExportData::Internal => {
                vec![]
            }
            XExportData::Array(x) => {
                for x in x.values.iter_mut() {
                    x.link_enumerate(all)
                }
            }
            XExportData::Dictionary(x) => {
                for x in x.values.iter_mut() {
                    x.link_enumerate(all)
                }
            }
            XExportData::Class(x) => {
                for x in x.values.iter_mut() {
                    x.link_enumerate(all)
                }
            }
            XExportData::Enumerate(x) => x.link_enumerate_head(),
        }
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
