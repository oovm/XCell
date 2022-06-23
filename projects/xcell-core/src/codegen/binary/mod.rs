use super::*;
use xcell_errors::XResult;
use xcell_types::{ByteOrder, StreamWriter};

impl BinaryCodegen {
    pub fn write_binary(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        let mut file = File::create(path)?;
        let rows = table.data.row_len();
        file.write_u32::<LittleEndian>(rows as u32)?;
        for row in table.data.rows_iter() {
            for item in row {
                item.write_to(&mut file, ByteOrder::LittleEndian)?
            }
        }
        Ok(())
    }
}
