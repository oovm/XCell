use itertools::Itertools;
use serde::Serialize;

use xcell_errors::for_3rd::{Datelike, Timelike, Utc, Zero};

use crate::{
    ArrayDescription, ArrayKind, BooleanDescription, ColorDescription, DecimalDescription, DecimalKind, IntegerDescription,
    IntegerKind, StringDescription, TimeDescription, XCellTyped, XCellValue,
};

mod default;

#[derive(Serialize)]
pub struct CSharpReader {
    pub is_vector: bool,
    pub field: String,
    pub function: String,
}

#[derive(Serialize)]
pub struct CSharpWriter {
    pub is_vector: bool,
    pub field: String,
    pub cast: String,
    pub properties: Vec<String>,
}

impl XCellValue {
    pub fn csharp_now() -> String {
        let now = Utc::now();
        format!(
            "new DateTime({year}, {month}, {day}, {hour}, {minute}, {second})",
            year = now.year(),
            month = now.month(),
            day = now.day(),
            hour = now.hour(),
            minute = now.minute(),
            second = now.second()
        )
    }
}

impl XCellTyped {
    pub fn as_csharp_type(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "bool".to_string(),
            XCellTyped::Integer(v) => v.kind.as_csharp_type().to_string(),
            XCellTyped::Decimal(v) => v.kind.as_csharp_type().to_string(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Time(_) => "DateTime".to_string(),
            XCellTyped::Color(_) => "Color32".to_string(),
            XCellTyped::Enumerate(v) => v.typing.to_owned(),
            XCellTyped::Array(v) => v.as_csharp_type().to_string(),
            XCellTyped::Vector(v) => format!("List<{}>", v.typing.as_csharp_type()),
        }
    }
}
