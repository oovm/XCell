use crate::XCellValue;

impl From<f64> for XCellValue {
    fn from(value: f64) -> Self {
        XCellValue::Float64(value.clone())
    }
}

impl From<&f64> for XCellValue {
    fn from(value: &f64) -> Self {
        XCellValue::Float64(value.clone())
    }
}
