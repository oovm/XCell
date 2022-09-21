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

    pub fn write_binary(&self, table: &XTable, output: &Path) -> XResult<()> {
        let mut file = File::create(output)?;
        let data = table.data.link_enumerate(&table.path).result(|e| log::error!("{e}"))?;
        match &data {
            XExportData::Array(_) | XExportData::Dictionary(_) | XExportData::Enumerate(_) => {
                let rows = data.rows_count();
                (rows as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
                for row in data.rows() {
                    for item in &row.data {
                        item.write_to(&mut file, ByteOrder::LittleEndian)?
                    }
                }
            }
            XExportData::Class(v) => {
                for item in &v.data {
                    item.write_to(&mut file, ByteOrder::LittleEndian)?
                }
            }
            XExportData::Language(_) => {
                todo!()
            }
        }
        Ok(())
    }
}
