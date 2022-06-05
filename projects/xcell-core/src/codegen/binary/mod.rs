use super::*;

impl BinaryCodegen {
    pub fn write_binary(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
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

impl XCellValue {
    pub fn write_binary(&self, f: &mut impl Write) -> std::io::Result<()> {
        match self {
            XCellValue::Boolean(v) => match v {
                true => f.write_u8(0),
                false => f.write_u8(1),
            },
            XCellValue::Integer8(v) => f.write_i8(*v),
            XCellValue::Integer16(v) => f.write_i16::<LittleEndian>(*v),
            XCellValue::Integer32(v) => f.write_i32::<LittleEndian>(*v),
            XCellValue::Integer64(v) => f.write_i64::<LittleEndian>(*v),
            XCellValue::Unsigned8(v) => f.write_u8(*v),
            XCellValue::Unsigned16(v) => f.write_u16::<LittleEndian>(*v),
            XCellValue::Unsigned32(v) => f.write_u32::<LittleEndian>(*v),
            XCellValue::Unsigned64(v) => f.write_u64::<LittleEndian>(*v),
            XCellValue::Float32(_) => {
                todo!()
            }
            XCellValue::Float64(_) => {
                todo!()
            }
            XCellValue::String(_) => {
                todo!()
            }
            XCellValue::Color(_) => {
                todo!()
            }
            XCellValue::Custom(_) => {
                todo!()
            }
        }
    }
}
