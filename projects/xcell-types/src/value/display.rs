use super::*;

impl Display for XCellValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XCellValue::Boolean(v) => {
                write!(f, "{v}")
            }
            XCellValue::Integer8(v) => {
                write!(f, "{v}")
            }
            XCellValue::Integer16(v) => {
                write!(f, "{v}")
            }
            XCellValue::Integer32(v) => {
                write!(f, "{v}")
            }
            XCellValue::Integer64(v) => {
                write!(f, "{v}")
            }
            XCellValue::Unsigned8(v) => {
                write!(f, "{v}")
            }
            XCellValue::Unsigned16(v) => {
                write!(f, "{v}")
            }
            XCellValue::Unsigned32(v) => {
                write!(f, "{v}")
            }
            XCellValue::Unsigned64(v) => {
                write!(f, "{v}")
            }
            XCellValue::Float32(v) => {
                write!(f, "{v}")
            }
            XCellValue::Float64(v) => {
                write!(f, "{v}")
            }
            XCellValue::String(v) => {
                write!(f, "{v}")
            }
            XCellValue::Color(v) => {
                write!(f, "{v}")
            }
            XCellValue::Vector2(_) => {
                todo!()
            }
            XCellValue::Vector3(_) => {
                todo!()
            }
            XCellValue::Vector4(_) => {
                todo!()
            }
            XCellValue::Color4(_) => {
                todo!()
            }
            XCellValue::Quaternion4(_) => {
                todo!()
            }
            XCellValue::Vector(v) => {
                println!("{v:?}");
                Ok(())
            }
            XCellValue::Enumerate(v) => {
                write!(f, "{v}")
            }
        }
    }
}
