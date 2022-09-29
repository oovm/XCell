use crate::EnumerateManager;

use super::*;

impl WorkspaceManager {
    pub fn link_enumerate(&mut self) -> Vec<XError> {
        self.enumerates.link_enumerate()
    }
}

impl EnumerateManager {
    pub fn link_enumerate(&mut self) -> Vec<XError> {
        let mut errors = vec![];
        for item in self.enumerate.values_mut() {
            assert_eq!(item.headers.len(), item.data.len());
            for define in item.headers.iter_mut() {
                if let Err(e) = define.link_enumerate(self) {
                    errors.push(e);
                }
            }
            for define in item.data.iter_mut() {
                if let Err(e) = define.link_enumerate(self) {
                    errors.push(e);
                }
            }
        }
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

impl XDataItem {
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

fn link_enumerate_data_cell(headers: &[XCellHeader], index: usize, data: &mut XCellValue) -> XResult<()> {
    match headers.get(index) {
        Some(s) => data.link_enumerate(&s.typing),
        None => Err(XError::table_error("not found")),
    }
}
