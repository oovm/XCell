use super::*;

impl BinaryCodegen {
    pub fn write_binary(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let rows = table.data.row_len();
        file.write_u32::<LittleEndian>(rows as u32)?;
        for row in table.data.rows_iter() {}
        Ok(())
    }
}
