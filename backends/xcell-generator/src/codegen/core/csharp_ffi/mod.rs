use itertools::Itertools;
use serde::{Deserialize, Serialize};

use xcell_core::{for_3rd::{Datelike, Timelike, Utc, Zero}, ArrayDescription, ArrayKind, BooleanDescription, ColorDescription, DecimalDescription, DecimalKind, IntegerDescription, IntegerKind, ListDescription, MapDescription, ReferenceDescription, StringDescription, TimeDescription, XCellTyped, XCellValue};

mod default;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CSharpReader {
    pub is_vector: bool,
    pub field: String,
    pub function: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// 返回当前 XCell 类型对应的 C# 类型名称
    pub fn as_csharp_type(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "bool".to_string(),
            XCellTyped::Integer(v) => v.kind.as_csharp_type().to_string(),
            XCellTyped::Decimal(v) => v.kind.as_csharp_type().to_string(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Time(_) => "DateTime".to_string(),
            XCellTyped::Color(_) => "Color32".to_string(),
            XCellTyped::Enumerate(v) => v.name.to_owned(),
            XCellTyped::Array(v) => v.as_csharp_type().to_string(),
            XCellTyped::Vector(v) => format!("List<{}>", v.get_type().as_csharp_type()),
            XCellTyped::Reference(_) => "int".to_string(),
            XCellTyped::List(v) => v.as_csharp_type(),
            XCellTyped::Map(v) => v.as_csharp_type(),
            XCellTyped::Optional(v) => format!("{}?", v.element_type.as_csharp_type()),
            XCellTyped::Unknown => "object".to_string(),
        }
    }
}

impl ReferenceDescription {
    /// 返回引用类型对应的 C# 类型名称
    pub fn as_csharp_type(&self) -> &'static str {
        "int"
    }
}

impl ListDescription {
    /// 返回列表类型对应的 C# 类型名称
    pub fn as_csharp_type(&self) -> String {
        format!("List<{}>", self.element_type.as_csharp_type())
    }
}

impl MapDescription {
    /// 返回映射类型对应的 C# 类型名称
    pub fn as_csharp_type(&self) -> String {
        format!("Dictionary<{}, {}>", self.key_type.as_csharp_type(), self.value_type.as_csharp_type())
    }
}
