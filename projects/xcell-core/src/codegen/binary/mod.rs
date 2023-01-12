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

    pub fn write_binary(&self, table: &XExportData, output: &Path) -> XResult<()> {
        match table {
            XExportData::Internal => {}
            XExportData::List(v) => {
                let mut file = File::create(output)?;
                (v.length() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
                for row in v.values() {
                    for item in &row.data {
                        item.write_to(&mut file, ByteOrder::LittleEndian)?
                    }
                }
            }
            XExportData::Dict(v) => {
                let mut file = File::create(output)?;
                (v.length() as u32).write_to(&mut file, ByteOrder::LittleEndian)?;
                for row in v.values() {
                    for item in &row.data {
                        item.write_to(&mut file, ByteOrder::LittleEndian)?
                    }
                }
            }
            XExportData::Class(v) => {
                let mut file = File::create(output)?;
                for item in &v.items {
                    item.default.write_to(&mut file, ByteOrder::LittleEndian)?
                }
            }
            XExportData::Enumerate(_) => {}
        }
        Ok(())
    }
}
