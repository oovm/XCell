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
            assert_eq!(item.headers.len(), item.lines.len());
            for define in item.headers.iter_mut() {
                if let Err(e) = define.link_enumerate(self) {
                    errors.push(e);
                }
            }
            for define in item.lines.iter_mut() {
                errors.extend(define.link_enumerate(&item.headers))
            }
        }
        errors
    }
}

impl XCellHeader {
    pub fn link_enumerate(&mut self, all: &EnumerateManager) -> XResult<()> {
        let define = match self.typing.mut_enumerate() {
            Some(s) => s,
            None => return Ok(()), // skip non enum
        };
        match all.define.get(&define.name) {
            Some(v) => {
                *define = v.clone();
                Ok(())
            }
            None => Err(XError::runtime_error(format!("未知的枚举类 `{}`", &define.name)).with_x(self.column)),
        }
    }
}

impl XDataLine {
    pub fn link_enumerate(&mut self, headers: &[XCellHeader]) -> Vec<XError> {
        assert_eq!(self.data.len(), headers.len());
        let mut errors = vec![];
        for (value, typing) in self.data.iter_mut().zip(headers.iter()) {
            if let Err(e) = value.link_enumerate(&typing.typing) {
                errors.push(e);
            }
        }
        errors
    }
}
