use super::*;

impl CsvCodegen {
    pub fn write_csv(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let rows = table.data.row_len();
        file.write_u32::<LittleEndian>(rows as u32)?;
        for row in table.data.rows_iter() {
            for item in row {
                item.write_binary(&mut file)?
            }
        }
        Ok(())
    }
}
