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
                    errors.push(e);
                }
            }
            for define in item.lines.iter_mut() {
                errors.extend(define.link_enumerate(&item.headers))
            }
        }
        for item in self.list.values_mut() {
            for define in item.headers.iter_mut() {
                if let Err(e) = EnumerateLink::link_enumerate(define, &self.define) {
                    errors.push(e);
                }
            }
            for define in item.mapping.values_mut() {
                errors.extend(define.link_enumerate(&item.headers))
            }
        }
        for item in self.dict.values_mut() {
            for define in item.headers.iter_mut() {
                if let Err(e) = EnumerateLink::link_enumerate(define, &self.define) {
                    errors.push(e);
                }
            }
            for define in item.mapping.values_mut() {
                errors.extend(define.link_enumerate(&item.headers))
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
        if self.data.len() != headers.len() {
            return vec![XError::runtime_error(format!("字段数量和类型数量不一致"))];
        }
        let mut errors = vec![];
        for (value, typing) in self.data.iter_mut().zip(headers.iter()) {
            if let Err(e) = value.link_enumerate(&typing.typing) {
                errors.push(e);
            }
        }
        errors
    }
}
