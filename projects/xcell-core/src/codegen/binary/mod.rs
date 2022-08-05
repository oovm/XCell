use xcell_errors::XResult;
use xcell_types::{ByteOrder, StreamWriter};

use super::*;

impl BinaryCodegen {
    pub fn write_binary(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        let mut file = File::create(path)?;
        let rows = table.data.rows_count();
        (rows as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
        for row in table.data.rows_iter() {
            for item in row {
                item.write_to(&mut file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }
}
