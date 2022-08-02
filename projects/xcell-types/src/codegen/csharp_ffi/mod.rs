use serde::Serialize;

use xcell_errors::for_3rd::{Datelike, Timelike, Zero};

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

impl XCellValue {
    pub fn csharp() {}
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
            XCellTyped::Vector(_) => {
                todo!()
            }
        }
    }
}
