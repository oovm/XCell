use crate::XError;

mod unity;

pub struct UnityCodegen {}

impl UnityCodegen {
    pub fn write_xml(&self, f: impl std::io::Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_bin(&self, f: impl std::io::Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp(&self, f: impl std::io::Write) -> Result<(), XError> {
        todo!()
    }
}
