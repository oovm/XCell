use itertools::Itertools;
use xcell_core::{
    ArrayDescription, ArrayKind, DecimalKind, IntegerKind, ListDescription, MapDescription,
    ReferenceDescription, XCellTyped, XCellValue,
    for_3rd::{Datelike, Timelike, Utc},
};

/// 返回当前时间的 TypeScript Date 初始化字符串
pub fn typescript_now() -> String {
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

/// 为类型提供 TypeScript 类型名称转换的扩展 trait
pub trait AsTypeScriptType {
    /// 返回当前类型对应的 TypeScript 类型名称
    fn as_typescript_type(&self) -> String;
}

/// 为类型提供 TypeScript 默认值转换的扩展 trait
pub trait AsTypeScriptDefault {
    /// 返回当前类型对应的 TypeScript 默认值字符串
    fn as_typescript_default(&self) -> String;
}

/// 为类型提供 TypeScript 值转换的扩展 trait
pub trait AsTypeScriptValue {
    /// 返回当前值对应的 TypeScript 值字符串
    fn as_typescript_value(&self) -> String;
}

impl AsTypeScriptType for XCellTyped {
    fn as_typescript_type(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "boolean".to_string(),
            XCellTyped::Integer(v) => v.kind.as_typescript_type(),
            XCellTyped::Decimal(v) => v.kind.as_typescript_type(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Time(_) => "Date".to_string(),
            XCellTyped::Color(_) => "{ r: number; g: number; b: number; a: number }".to_string(),
            XCellTyped::Enumerate(v) => v.name.to_owned(),
            XCellTyped::Array(v) => v.as_typescript_type(),
            XCellTyped::Vector(v) => format!("Array<{}>", v.get_type().as_typescript_type()),
            XCellTyped::Reference(_) => "number".to_string(),
            XCellTyped::List(v) => v.as_typescript_type(),
            XCellTyped::Map(v) => v.as_typescript_type(),
            XCellTyped::Optional(v) => format!("{} | null", v.element_type.as_typescript_type()),
            XCellTyped::Unknown => "any".to_string(),
        }
    }
}

impl AsTypeScriptType for IntegerKind {
    fn as_typescript_type(&self) -> String {
        "number".to_string()
    }
}

impl AsTypeScriptType for DecimalKind {
    fn as_typescript_type(&self) -> String {
        "number".to_string()
    }
}

impl AsTypeScriptType for ReferenceDescription {
    fn as_typescript_type(&self) -> String {
        "number".to_string()
    }
}

impl AsTypeScriptType for ListDescription {
    fn as_typescript_type(&self) -> String {
        format!("{}[]", self.element_type.as_typescript_type())
    }
}

impl AsTypeScriptType for MapDescription {
    fn as_typescript_type(&self) -> String {
        format!(
            "Record<{}, {}>",
            self.key_type.as_typescript_type(),
            self.value_type.as_typescript_type()
        )
    }
}

impl AsTypeScriptType for ArrayDescription {
    fn as_typescript_type(&self) -> String {
        match self.kind {
            ArrayKind::Vector2 => "[number, number]".to_string(),
            ArrayKind::Vector3 => "[number, number, number]".to_string(),
            ArrayKind::Vector4 => "[number, number, number, number]".to_string(),
            ArrayKind::Quaternion4 => "[number, number, number, number]".to_string(),
        }
    }
}

impl AsTypeScriptDefault for XCellTyped {
    fn as_typescript_default(&self) -> String {
        match self {
            XCellTyped::Boolean(v) => {
                if v.default {
                    "true".to_string()
                } else {
                    "".to_string()
                }
            }
            XCellTyped::Integer(_) => "".to_string(),
            XCellTyped::Decimal(_) => "".to_string(),
            XCellTyped::String(v) => {
                if v.default.is_empty() {
                    "\"\"".to_string()
                } else {
                    format!("{:?}", v.default)
                }
            }
            XCellTyped::Time(_) => "new Date()".to_string(),
            XCellTyped::Color(_) => "{ r: 0, g: 0, b: 0, a: 255 }".to_string(),
            XCellTyped::Enumerate(v) => v.default.to_string(),
            XCellTyped::Array(_) => "[]".to_string(),
            XCellTyped::Vector(_) => "[]".to_string(),
            XCellTyped::Reference(v) => v.default.clone().unwrap_or_else(|| "0".to_string()),
            XCellTyped::List(_) => "[]".to_string(),
            XCellTyped::Map(_) => "{}".to_string(),
            XCellTyped::Optional(_) => "null".to_string(),
            XCellTyped::Unknown => "null".to_string(),
        }
    }
}

impl AsTypeScriptValue for XCellValue {
    fn as_typescript_value(&self) -> String {
        match self {
            XCellValue::Boolean(v) => v.to_string(),
            XCellValue::Integer8(v) => v.to_string(),
            XCellValue::Integer16(v) => v.to_string(),
            XCellValue::Integer32(v) => v.to_string(),
            XCellValue::Integer64(v) => v.to_string(),
            XCellValue::Unsigned8(v) => v.to_string(),
            XCellValue::Unsigned16(v) => v.to_string(),
            XCellValue::Unsigned32(v) => v.to_string(),
            XCellValue::Unsigned64(v) => v.to_string(),
            XCellValue::Float32(v) => v.to_string(),
            XCellValue::Float64(v) => v.to_string(),
            XCellValue::Vector2(v) => format!("[{}, {}]", v[0], v[1]),
            XCellValue::Vector3(v) => format!("[{}, {}, {}]", v[0], v[1], v[2]),
            XCellValue::Vector4(v) => format!("[{}, {}, {}, {}]", v[0], v[1], v[2], v[3]),
            XCellValue::Quaternion4(v) => format!("[{}, {}, {}, {}]", v[0], v[1], v[2], v[3]),
            XCellValue::String(s) => {
                format!("\"{}\"", s)
            }
            XCellValue::Color(c) => {
                format!(
                    "{{ r: {}, g: {}, b: {}, a: {} }}",
                    (c.r * 255.0) as u8,
                    (c.g * 255.0) as u8,
                    (c.b * 255.0) as u8,
                    (c.a * 255.0) as u8
                )
            }
            XCellValue::Enumerate(_) => "0".to_string(),
            XCellValue::Vector(v) => {
                format!("[{}]", v.iter().map(|x| x.as_typescript_value()).join(", "))
            }
            XCellValue::Reference(v) => v.to_string(),
            XCellValue::Map(v) => format!("{:?}", v),
            XCellValue::Optional(v) => match v {
                Some(inner) => inner.as_typescript_value(),
                None => "null".to_string(),
            },
        }
    }
}
