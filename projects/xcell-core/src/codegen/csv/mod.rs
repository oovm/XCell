use std::io::Write;

use crate::{codegen::CsvCodegen, XCellTable, XError};

impl CsvCodegen {
    pub fn write_csv(&self, table: &XCellTable, f: impl Write) -> Result<(), XError> {
        todo!()
    }
}
