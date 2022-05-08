use std::io::Write;

use crate::{codegen::UnityCodegen, XCellTable, XError};

impl UnityCodegen {
    pub fn write_xml(&self, table: &XCellTable, f: impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_bin(&self, table: &XCellTable, f: impl Write) -> Result<(), XError> {
        todo!()
    }
    pub fn write_csharp(&self, table: &XCellTable, f: impl Write) -> Result<(), XError> {
        todo!()
    }
}
