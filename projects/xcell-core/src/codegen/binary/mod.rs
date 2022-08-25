use super::*;

pub struct BinaryWriter {
    bit7_encode: bool,
}

impl Default for BinaryWriter {
    fn default() -> Self {
        Self { bit7_encode: true }
    }
}

impl BinaryWriter {
    pub fn set_bit7(&mut self, enable: bool) {
        self.bit7_encode = enable;
    }

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
