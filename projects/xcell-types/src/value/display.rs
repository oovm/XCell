use super::*;

impl Display for XCellValueKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XCellValueKind::Boolean(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Integer8(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Integer16(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Integer32(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Integer64(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Unsigned8(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Unsigned16(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Unsigned32(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Unsigned64(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Float32(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Float64(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::String(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Color(v) => {
                write!(f, "{v}")
            }
            XCellValueKind::Vector2(_) => {
                todo!()
            }
            XCellValueKind::Vector3(_) => {
                todo!()
            }
            XCellValueKind::Vector4(_) => {
                todo!()
            }
            XCellValueKind::Color4(_) => {
                todo!()
            }
            XCellValueKind::Quaternion4(_) => {
                todo!()
            }
            XCellValueKind::Vector(v) => {
                println!("{v:?}");
                Ok(())
            }
            XCellValueKind::Enumerate(v) => {
                write!(f, "{v}")
            }
        }
    }
}
