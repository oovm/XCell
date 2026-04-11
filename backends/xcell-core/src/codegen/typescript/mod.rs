use crate::{ListDescription, MapDescription, ReferenceDescription, XCellTyped};
use itertools::Itertools;

impl XCellTyped {
    /// 返回当前 XCell 类型对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "boolean".to_string(),
            XCellTyped::Integer(v) => v.kind.as_typescript_type().to_string(),
            XCellTyped::Decimal(v) => v.kind.as_typescript_type().to_string(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Time(_) => "Date".to_string(),
            XCellTyped::Color(_) => "{ r: number; g: number; b: number; a: number }".to_string(),
            XCellTyped::Enumerate(v) => v.name.to_owned(),
            XCellTyped::Array(v) => v.as_typescript_type().to_string(),
            XCellTyped::Vector(v) => format!("Array<{}>", v.get_type().as_typescript_type()),
            XCellTyped::Reference(_) => "number".to_string(),
            XCellTyped::List(v) => v.as_typescript_type(),
            XCellTyped::Map(v) => v.as_typescript_type(),
            XCellTyped::Optional(v) => format!("{} | null", v.element_type.as_typescript_type()),
            XCellTyped::Unknown => "any".to_string(),
        }
    }

    /// 返回当前 XCell 类型对应的 TypeScript 默认值
    pub fn as_typescript_default(&self) -> String {
        match self {
            XCellTyped::Boolean(v) => {
                if v.default {
                    "true".to_string()
                }
                else {
                    "".to_string()
                }
            }
            XCellTyped::Integer(_) => "".to_string(),
            XCellTyped::Decimal(_) => "".to_string(),
            XCellTyped::String(v) => {
                if v.default.is_empty() {
                    "\"\"".to_string()
                }
                else {
                    format!("{:?}", v.default)
                }
            }
            XCellTyped::Time(_) => "new Date()".to_string(),
            XCellTyped::Color(_) => "{ r: 0, g: 0, b: 0, a: 255 }".to_string(),
            XCellTyped::Enumerate(v) => v.default.to_string(),
            XCellTyped::Array(_) => "[]".to_string(),
            XCellTyped::Vector(_) => "[]".to_string(),
            XCellTyped::Reference(v) => v.default.map_or("0".to_string(), |d| d.to_string()),
            XCellTyped::List(_) => "[]".to_string(),
            XCellTyped::Map(_) => "{}".to_string(),
            XCellTyped::Optional(_) => "null".to_string(),
            XCellTyped::Unknown => "null".to_string(),
        }
    }
}

impl ReferenceDescription {
    /// 返回引用类型对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> &'static str {
        "number"
    }
}

impl ListDescription {
    /// 返回列表类型对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> String {
        format!("{}[]", self.element_type.as_typescript_type())
    }
}

impl MapDescription {
    /// 返回映射类型对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> String {
        format!("Record<{}, {}>", self.key_type.as_typescript_type(), self.value_type.as_typescript_type())
    }
}

impl crate::XCellValue {
    /// 返回当前时间的 TypeScript Date 初始化字符串
    pub fn typescript_now() -> String {
        use crate::for_3rd::{Datelike, Timelike, Utc};
        let now = Utc::now();
        format!(
            "new Date({year}, {month}, {day}, {hour}, {minute}, {second})",
            year = now.year(),
            month = now.month() - 1,
            day = now.day(),
            hour = now.hour(),
            minute = now.minute(),
            second = now.second()
        )
    }

    /// 返回当前 XCell 值对应的 TypeScript 值字符串
    pub fn as_typescript_value(&self) -> String {
        match self {
            crate::XCellValue::Boolean(v) => v.to_string(),
            crate::XCellValue::Integer8(v) => v.to_string(),
            crate::XCellValue::Integer16(v) => v.to_string(),
            crate::XCellValue::Integer32(v) => v.to_string(),
            crate::XCellValue::Integer64(v) => v.to_string(),
            crate::XCellValue::Unsigned8(v) => v.to_string(),
            crate::XCellValue::Unsigned16(v) => v.to_string(),
            crate::XCellValue::Unsigned32(v) => v.to_string(),
            crate::XCellValue::Unsigned64(v) => v.to_string(),
            crate::XCellValue::Float32(v) => v.to_string(),
            crate::XCellValue::Float64(v) => v.to_string(),
            crate::XCellValue::Vector2(v) => format!("[{}, {}]", v[0], v[1]),
            crate::XCellValue::Vector3(v) => format!("[{}, {}, {}]", v[0], v[1], v[2]),
            crate::XCellValue::Vector4(v) => format!("[{}, {}, {}, {}]", v[0], v[1], v[2], v[3]),
            crate::XCellValue::Quaternion4(v) => format!("[{}, {}, {}, {}]", v[0], v[1], v[2], v[3]),
            crate::XCellValue::String(s) => {
                format!("\"{}\"", s)
            }
            crate::XCellValue::Color(c) => {
                format!(
                    "{{ r: {}, g: {}, b: {}, a: {} }}",
                    (c.r * 255.0) as u8,
                    (c.g * 255.0) as u8,
                    (c.b * 255.0) as u8,
                    (c.a * 255.0) as u8
                )
            }
            crate::XCellValue::Enumerate(_) => "0".to_string(),
            crate::XCellValue::Vector(v) => {
                format!("[{}]", v.iter().map(|x| x.as_typescript_value()).join(", "))
            }
            crate::XCellValue::Reference(v) => v.to_string(),
            crate::XCellValue::Map(v) => format!("{:?}", v),
            crate::XCellValue::Optional(v) => match v {
                Some(inner) => inner.as_typescript_value(),
                None => "null".to_string(),
            },
        }
    }
}
