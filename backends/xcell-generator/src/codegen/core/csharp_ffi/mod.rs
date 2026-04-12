use itertools::Itertools;
use serde::{Deserialize, Serialize};
use xcell_core::{
    ArrayDescription, ArrayKind, BooleanDescription, ColorDescription, DecimalDescription,
    DecimalKind, IntegerDescription, IntegerKind, ListDescription, MapDescription,
    ReferenceDescription, StringDescription, TimeDescription, XCellTyped, XCellValue,
    for_3rd::{Datelike, Timelike, Utc, Zero},
};

/// C# 二进制读取器配置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CSharpReader {
    /// 是否为向量类型
    pub is_vector: bool,
    /// 字段名称
    pub field: String,
    /// 读取函数表达式
    pub function: String,
}

/// C# 二进制写入器配置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CSharpWriter {
    /// 是否为向量类型
    pub is_vector: bool,
    /// 字段名称
    pub field: String,
    /// 类型转换表达式
    pub cast: String,
    /// 属性列表
    pub properties: Vec<String>,
}

/// 返回当前时间的 C# DateTime 初始化字符串
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

/// 为类型提供 C# 类型名称转换的扩展 trait
pub trait AsCSharpType {
    /// 返回当前类型对应的 C# 类型名称
    fn as_csharp_type(&self) -> String;
}

/// 为类型提供 C# 默认值转换的扩展 trait
pub trait AsCSharpDefault {
    /// 返回当前类型对应的 C# 默认值字符串
    fn as_csharp_default(&self) -> String;
}

/// 为类型提供 C# 二进制读取器配置的扩展 trait
pub trait CSharpBinaryReader {
    /// 创建 C# 二进制读取器配置
    fn make_cs_binary_reader(&self, field: &str) -> CSharpReader;
}

/// 为类型提供 C# 二进制写入器配置的扩展 trait
pub trait CSharpBinaryWriter {
    /// 创建 C# 二进制写入器配置
    fn make_cs_binary_writer(&self, field: &str) -> CSharpWriter;
}

/// 为类型提供 C# 读取表达式转换的扩展 trait
pub trait AsCSharpReader {
    /// 返回 C# 二进制读取表达式
    fn as_csharp_reader(&self) -> String;
}

/// 为类型提供 C# 值转换的扩展 trait
pub trait AsCSharpValue {
    /// 返回当前值对应的 C# 值字符串
    fn as_csharp_value(&self) -> String;
}

impl AsCSharpType for XCellTyped {
    fn as_csharp_type(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "bool".to_string(),
            XCellTyped::Integer(v) => v.kind.as_csharp_type(),
            XCellTyped::Decimal(v) => v.kind.as_csharp_type(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Time(_) => "DateTime".to_string(),
            XCellTyped::Color(_) => "Color32".to_string(),
            XCellTyped::Enumerate(v) => v.name.to_owned(),
            XCellTyped::Array(v) => v.as_csharp_type(),
            XCellTyped::Vector(v) => format!("List<{}>", v.get_type().as_csharp_type()),
            XCellTyped::Reference(_) => "int".to_string(),
            XCellTyped::List(v) => v.as_csharp_type(),
            XCellTyped::Map(v) => v.as_csharp_type(),
            XCellTyped::Optional(v) => format!("{}?", v.element_type.as_csharp_type()),
            XCellTyped::Unknown => "object".to_string(),
        }
    }
}

impl AsCSharpType for IntegerKind {
    fn as_csharp_type(&self) -> String {
        match self {
            IntegerKind::Integer8 => "sbyte".to_string(),
            IntegerKind::Integer16 => "short".to_string(),
            IntegerKind::Integer32 => "int".to_string(),
            IntegerKind::Integer64 => "long".to_string(),
            IntegerKind::Unsigned8 => "byte".to_string(),
            IntegerKind::Unsigned16 => "ushort".to_string(),
            IntegerKind::Unsigned32 => "uint".to_string(),
            IntegerKind::Unsigned64 => "ulong".to_string(),
        }
    }
}

impl AsCSharpType for DecimalKind {
    fn as_csharp_type(&self) -> String {
        match self {
            DecimalKind::Float32 => "float".to_string(),
            DecimalKind::Float64 => "double".to_string(),
            DecimalKind::Decimal128 => "decimal".to_string(),
        }
    }
}

impl AsCSharpType for ReferenceDescription {
    fn as_csharp_type(&self) -> String {
        "int".to_string()
    }
}

impl AsCSharpType for ListDescription {
    fn as_csharp_type(&self) -> String {
        format!("List<{}>", self.element_type.as_csharp_type())
    }
}

impl AsCSharpType for MapDescription {
    fn as_csharp_type(&self) -> String {
        format!(
            "Dictionary<{}, {}>",
            self.key_type.as_csharp_type(),
            self.value_type.as_csharp_type()
        )
    }
}

impl AsCSharpType for ArrayDescription {
    fn as_csharp_type(&self) -> String {
        match self.kind {
            ArrayKind::Vector2 => "Vector2".to_string(),
            ArrayKind::Vector3 => "Vector3".to_string(),
            ArrayKind::Vector4 => "Vector4".to_string(),
            ArrayKind::Quaternion4 => "Quaternion4".to_string(),
        }
    }
}

impl AsCSharpReader for IntegerKind {
    fn as_csharp_reader(&self) -> String {
        match self {
            IntegerKind::Integer8 => "r.ReadByte()".to_string(),
            IntegerKind::Integer16 => "r.ReadInt16()".to_string(),
            IntegerKind::Integer32 => "r.ReadInt32()".to_string(),
            IntegerKind::Integer64 => "r.ReadInt64()".to_string(),
            IntegerKind::Unsigned8 => "r.ReadSByte()".to_string(),
            IntegerKind::Unsigned16 => "r.ReadUInt16()".to_string(),
            IntegerKind::Unsigned32 => "r.ReadUInt32()".to_string(),
            IntegerKind::Unsigned64 => "r.ReadUInt64()".to_string(),
        }
    }
}

impl AsCSharpReader for XCellTyped {
    fn as_csharp_reader(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "r.ReadBoolean()".to_string(),
            XCellTyped::Integer(v) => v.kind.as_csharp_reader(),
            XCellTyped::Decimal(v) => match v.kind {
                DecimalKind::Float32 => "r.ReadSingle()".to_string(),
                DecimalKind::Float64 => "r.ReadDouble()".to_string(),
                DecimalKind::Decimal128 => "r.ReadDecimal()".to_string(),
            },
            XCellTyped::String(_) => "r.ReadString()".to_string(),
            XCellTyped::Time(_) => "new DateTime(r.ReadInt64(), DateTimeKind.Utc)".to_string(),
            XCellTyped::Color(_) => {
                "new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())".to_string()
            }
            XCellTyped::Enumerate(v) => {
                format!("({}) {}", v.name, v.integer.as_csharp_reader())
            }
            XCellTyped::Array(v) => match v.kind {
                ArrayKind::Vector2 => {
                    "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"
                        .to_string()
                }
                ArrayKind::Vector3 => {
                    "new Vector3(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"
                        .to_string()
                }
                ArrayKind::Vector4 => {
                    "new Vector4(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"
                        .to_string()
                }
                ArrayKind::Quaternion4 => {
                    "new Quaternion4(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())"
                        .to_string()
                }
            },
            XCellTyped::Vector(_) => unreachable!(),
            XCellTyped::Reference(_) => "r.ReadInt32()".to_string(),
            XCellTyped::List(_) => unreachable!(),
            XCellTyped::Map(_) => unreachable!(),
            XCellTyped::Optional(_) => unreachable!(),
            XCellTyped::Unknown => "null".to_string(),
        }
    }
}

impl CSharpBinaryReader for XCellTyped {
    fn make_cs_binary_reader(&self, field: &str) -> CSharpReader {
        match self {
            XCellTyped::Vector(v) => CSharpReader {
                is_vector: true,
                ..v.get_type().make_cs_binary_reader(field)
            },
            XCellTyped::List(v) => CSharpReader {
                is_vector: true,
                ..v.element_type.make_cs_binary_reader(field)
            },
            XCellTyped::Map(_) => CSharpReader {
                is_vector: false,
                function: "r.ReadString()".to_string(),
                field: field.to_string(),
            },
            XCellTyped::Optional(v) => v.element_type.make_cs_binary_reader(field),
            _ => CSharpReader {
                is_vector: false,
                function: self.as_csharp_reader(),
                field: field.to_string(),
            },
        }
    }
}

impl CSharpBinaryWriter for XCellTyped {
    fn make_cs_binary_writer(&self, field: &str) -> CSharpWriter {
        let properties = match self {
            XCellTyped::Time(_) => {
                vec![".Ticks".to_string()]
            }
            XCellTyped::Color(_) => {
                vec![
                    ".r".to_string(),
                    ".g".to_string(),
                    ".b".to_string(),
                    ".a".to_string(),
                ]
            }
            XCellTyped::Enumerate(_) => {
                return CSharpWriter {
                    is_vector: false,
                    field: field.to_string(),
                    cast: "(int) ".to_string(),
                    properties: vec!["".to_string()],
                };
            }
            XCellTyped::Vector(v) => {
                return CSharpWriter {
                    is_vector: true,
                    ..v.get_type().make_cs_binary_writer(field)
                };
            }
            XCellTyped::Reference(_) => {
                return CSharpWriter {
                    is_vector: false,
                    field: field.to_string(),
                    cast: "".to_string(),
                    properties: vec!["".to_string()],
                };
            }
            XCellTyped::List(v) => {
                return CSharpWriter {
                    is_vector: true,
                    ..v.element_type.make_cs_binary_writer(field)
                };
            }
            XCellTyped::Map(_) => {
                return CSharpWriter {
                    is_vector: false,
                    field: field.to_string(),
                    cast: "".to_string(),
                    properties: vec!["".to_string()],
                };
            }
            XCellTyped::Optional(v) => {
                return v.element_type.make_cs_binary_writer(field);
            }
            _ => vec!["".to_string()],
        };
        CSharpWriter {
            is_vector: false,
            field: field.to_string(),
            cast: "".to_string(),
            properties,
        }
    }
}

impl AsCSharpDefault for XCellTyped {
    fn as_csharp_default(&self) -> String {
        match self {
            XCellTyped::Boolean(v) => v.as_csharp_default(),
            XCellTyped::Integer(v) => v.as_csharp_default(),
            XCellTyped::Decimal(v) => v.as_csharp_default(),
            XCellTyped::String(v) => v.as_csharp_default(),
            XCellTyped::Time(v) => v.as_csharp_default(),
            XCellTyped::Color(v) => v.as_csharp_default(),
            XCellTyped::Enumerate(v) => v.default.to_string(),
            XCellTyped::Array(v) => v.as_csharp_default(),
            XCellTyped::Vector(v) => {
                if v.default.is_empty() {
                    return "new()".to_string();
                }
                let mut result = String::new();
                result.push_str("new () {");
                let joined = v.default.iter().map(|v| v.as_csharp_value()).join(", ");
                result.push_str(&joined);
                result.push_str("}");
                result
            }
            XCellTyped::Reference(v) => v.as_csharp_default(),
            XCellTyped::List(v) => v.as_csharp_default(),
            XCellTyped::Map(_) => "new()".to_string(),
            XCellTyped::Optional(v) => v.element_type.as_csharp_default(),
            XCellTyped::Unknown => "null".to_string(),
        }
    }
}

impl AsCSharpDefault for BooleanDescription {
    fn as_csharp_default(&self) -> String {
        match self.default {
            true => "true".to_string(),
            false => "".to_string(),
        }
    }
}

impl AsCSharpDefault for IntegerDescription {
    fn as_csharp_default(&self) -> String {
        if self.default.is_zero() {
            "".to_string()
        } else {
            self.default.to_string()
        }
    }
}

impl AsCSharpDefault for DecimalDescription {
    fn as_csharp_default(&self) -> String {
        if self.default.is_zero() {
            "".to_string()
        } else {
            self.default.to_string()
        }
    }
}

impl AsCSharpDefault for StringDescription {
    fn as_csharp_default(&self) -> String {
        if self.default.is_empty() {
            "\"\"".to_string()
        } else {
            format!("{:?}", self.default)
        }
    }
}

impl AsCSharpDefault for TimeDescription {
    fn as_csharp_default(&self) -> String {
        match &self.default {
            Some(s) => {
                format!(
                    "new DateTime({year}, {month}, {day}, {hour}, {minute}, {second})",
                    year = s.year(),
                    month = s.month(),
                    day = s.day(),
                    hour = s.hour(),
                    minute = s.minute(),
                    second = s.second()
                )
            }
            None => "new DateTime()".to_string(),
        }
    }
}

impl AsCSharpDefault for ColorDescription {
    fn as_csharp_default(&self) -> String {
        let [r, g, b, a] = self.default.to_rgba8();
        format!("new Color32({r}, {g}, {b}, {a})")
    }
}

impl AsCSharpDefault for ArrayDescription {
    fn as_csharp_default(&self) -> String {
        "new ()".to_string()
    }
}

impl AsCSharpDefault for ReferenceDescription {
    fn as_csharp_default(&self) -> String {
        match self.default {
            Some(v) => v.to_string(),
            None => "0".to_string(),
        }
    }
}

impl AsCSharpDefault for ListDescription {
    fn as_csharp_default(&self) -> String {
        if self.default.is_empty() {
            "new()".to_string()
        } else {
            let mut result = String::new();
            result.push_str("new () {");
            let joined = self.default.iter().map(|v| v.as_csharp_value()).join(", ");
            result.push_str(&joined);
            result.push_str("}");
            result
        }
    }
}

impl AsCSharpValue for XCellValue {
    fn as_csharp_value(&self) -> String {
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
            XCellValue::Float32(v) => format!("{}f", v),
            XCellValue::Float64(v) => format!("{}d", v),
            XCellValue::Vector2(v) => format!("new Vector2({}f, {}f)", v[0], v[1]),
            XCellValue::Vector3(v) => format!("new Vector3({}f, {}f, {}f)", v[0], v[1], v[2]),
            XCellValue::Vector4(v) => {
                format!("new Vector4({}f, {}f, {}f, {}f)", v[0], v[1], v[2], v[3])
            }
            XCellValue::Quaternion4(v) => {
                format!(
                    "new Quaternion4({}f, {}f, {}f, {}f)",
                    v[0], v[1], v[2], v[3]
                )
            }
            XCellValue::String(s) => {
                let mut result = String::new();
                result.push('"');
                result.push_str(s);
                result.push('"');
                result
            }
            XCellValue::Color(c) => {
                format!(
                    "new Color32({r}, {g}, {b}, {a})",
                    r = (c.r * 255.0) as u8,
                    g = (c.g * 255.0) as u8,
                    b = (c.b * 255.0) as u8,
                    a = (c.a * 255.0) as u8
                )
            }
            XCellValue::Enumerate(v) => v.clone(),
            XCellValue::Vector(v) => {
                format!(
                    "new () {{ {} }}",
                    v.iter().map(|x| x.as_csharp_value()).join(", ")
                )
            }
            XCellValue::Reference(v) => v.to_string(),
            XCellValue::Map(v) => format!("{:?}", v),
            XCellValue::Optional(v) => match v {
                Some(inner) => inner.as_csharp_value(),
                None => "null".to_string(),
            },
        }
    }
}
