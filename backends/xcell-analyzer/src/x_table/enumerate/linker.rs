use crate::DefineManager;

use super::*;

impl WorkspaceManager {
    pub fn link_enumerate(&mut self) -> Vec<XError> {
        self.defines.link_enumerate()
    }
}

impl DefineManager {
    pub fn link_enumerate(&mut self) -> Vec<XError> {
        let mut errors = vec![];
        for item in self.enumerate.values_mut() {
            for define in item.headers.iter_mut() {
                if let Err(e) = EnumerateLink::link_enumerate(define, &self.define) {
                    errors.push(e.with_path(&item.path));
                }
            }
            for define in item.lines.iter_mut() {
                for e in define.link_enumerate(&item.headers) {
                    errors.push(e.with_path(&item.path));
                }
            }
        }
        for item in self.list.values_mut() {
            for define in item.headers.iter_mut() {
                if let Err(e) = EnumerateLink::link_enumerate(define, &self.define) {
                    errors.push(e.with_path(&item.path));
                }
            }
            for define in item.mapping.values_mut() {
                for e in define.link_enumerate(&item.headers) {
                    errors.push(e.with_path(&item.path));
                }
            }
        }
        for item in self.dict.values_mut() {
            for define in item.headers.iter_mut() {
                if let Err(e) = EnumerateLink::link_enumerate(define, &self.define) {
                    errors.push(e.with_path(&item.path));
                }
            }
            for define in item.mapping.values_mut() {
                for e in define.link_enumerate(&item.headers) {
                    errors.push(e.with_path(&item.path));
                }
            }
        }
        errors
    }
    pub fn get_enumerate(&mut self, name: &str) -> Option<&mut XEnumerateData> {
        self.enumerate.get_mut(name)
    }
}

/// 枚举链接 trait
pub trait EnumerateLink {
    /// 链接枚举定义
    fn link_enumerate(&mut self, all: &BTreeMap<String, EnumerateDescription>) -> XResult<()>;
}

impl EnumerateLink for XCellHeader {
    fn link_enumerate(&mut self, all: &BTreeMap<String, EnumerateDescription>) -> XResult<()> {
        let define = match self.typing.mut_enumerate() {
            Some(s) => s,
            // 非枚举类型, 跳过
            None => return Ok(()),
        };
        match all.get(&define.name) {
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
        let valid_headers: Vec<_> = headers.iter().filter(|h| !matches!(h.typing, xcell_core::XCellTyped::Unknown)).collect();
        if self.data.len() != valid_headers.len() {
            return vec![XError::runtime_error(format!("字段数量和类型数量不一致: data={}, headers={}", self.data.len(), valid_headers.len())).with_y(self.row)];
        }
        let mut errors = vec![];
        for (value, typing) in self.data.iter_mut().zip(valid_headers.iter()) {
            if let Err(e) = value.link_enumerate(&typing.typing) {
                errors.push(e.with_xy(typing.column, self.row));
            }
        }
        errors
    }
}
