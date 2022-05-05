use crate::{codegen::UnityCodegen, XCellTable, XError};
use std::io::Write;

impl UnityCodegen for XCellTable {
    fn write_xml(&self, f: impl Write) -> Result<(), XError> {
        todo!()
    }

    fn write_bin(&self, f: impl Write) -> Result<(), XError> {
        todo!()
    }
}
