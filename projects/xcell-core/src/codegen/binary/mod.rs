use xcell_errors::{Validation, XResult};
use xcell_types::{ByteOrder, StreamWriter};

use super::*;

impl BinaryCodegen {
    pub fn write_binary(&self, table: &XCellTable, output: &Path) -> XResult<()> {
        let mut file = File::create(output)?;
        let data = table.data.link_enumerate().result(|e| log::error!("{e}"))?;
        let rows = data.rows_count();
        (rows as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
        for row in data.rows() {
            for item in &row.data {
                item.write_to(&mut file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }
}
