use super::*;

impl XCellTyped {
    /// 返回当前 XCell 类型对应的 C# 默认值字符串
    pub fn as_csharp_default(&self) -> String {
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
                format!("new () {{{}}}", v.default.iter().map(|v| v.as_csharp_value()).join(", "))
            }
            XCellTyped::Reference(v) => v.as_csharp_default(),
            XCellTyped::List(v) => v.as_csharp_default(),
            XCellTyped::Map(_) => "new()".to_string(),
            XCellTyped::Optional(v) => v.element_type.as_csharp_default(),
            XCellTyped::Unknown => "null".to_string(),
        }
    }
}

impl XCellValue {
    /// 返回当前 XCell 值对应的 C# 值字符串
    pub fn as_csharp_value(&self) -> String {
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
            XCellValue::Vector2(_) => {
                todo!()
            }
            XCellValue::Vector3(_) => {
                todo!()
            }
            XCellValue::Vector4(_) => {
                todo!()
            }
            XCellValue::Quaternion4(_) => {
                todo!()
            }
            XCellValue::String(s) => {
                format!("\"{}\"", s)
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
            XCellValue::Enumerate(_) => {
                todo!()
            }
            XCellValue::Vector(_) => {
                todo!()
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

impl BooleanDescription {
    /// 返回布尔类型对应的 C# 默认值字符串
    ///
    /// # 返回值
    /// 返回 C# 默认值字符串
    pub fn as_csharp_default(&self) -> String {
        match self.default {
            true => "true".to_string(),
            false => "".to_string(),
        }
    }
}

impl IntegerDescription {
    /// 返回整数类型对应的 C# 默认值字符串
    ///
    /// # 返回值
    /// 返回 C# 默认值字符串
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_zero() { "".to_string() } else { self.default.to_string() }
    }
}

impl DecimalDescription {
    /// 返回小数类型对应的 C# 默认值字符串
    ///
    /// # 返回值
    /// 返回 C# 默认值字符串
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_zero() { "".to_string() } else { self.default.to_string() }
    }
}

impl StringDescription {
    /// 返回字符串类型对应的 C# 默认值字符串
    ///
    /// # 返回值
    /// 返回 C# 默认值字符串
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_empty() { "\"\"".to_string() } else { format!("{:?}", self.default) }
    }
}

impl TimeDescription {
    /// 返回时间类型对应的 C# 默认值字符串
    ///
    /// # 返回值
    /// 返回 C# 默认值字符串
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

impl ColorDescription {
    /// 返回颜色类型对应的 C# 默认值字符串
    ///
    /// # 返回值
    /// 返回 C# 默认值字符串
    fn as_csharp_default(&self) -> String {
        let [r, g, b, a] = self.default.to_rgba8();
        format!("new Color32({r}, {g}, {b}, {a})")
    }
}

impl ArrayDescription {
    /// 返回数组类型对应的 C# 默认值字符串
    ///
    /// # 返回值
    /// 返回 C# 默认值字符串
    fn as_csharp_default(&self) -> String {
        "new ()".to_string()
    }
}

impl XCellTyped {
    /// 创建 C# 二进制写入器配置
    ///
    /// # 参数
    /// * `field` - 字段名称
    ///
    /// # 返回值
    /// 返回 CSharpWriter 配置结构体
    pub fn make_cs_binary_writer(&self, field: &str) -> CSharpWriter {
        let properties = match self {
            XCellTyped::Time(_) => vec![".Ticks".to_string()],
            XCellTyped::Color(_) => vec![".r".to_string(), ".g".to_string(), ".b".to_string(), ".a".to_string()],
            XCellTyped::Enumerate(_) => {
                return CSharpWriter {
                    is_vector: false,
                    field: field.to_string(),
                    cast: "(int) ".to_string(),
                    properties: vec!["".to_string()],
                };
            }
            XCellTyped::Vector(v) => return CSharpWriter { is_vector: true, ..v.get_type().make_cs_binary_writer(field) },
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
        CSharpWriter { is_vector: false, field: field.to_string(), cast: "".to_string(), properties }
    }

    /// 创建 C# 二进制读取器配置
    ///
    /// # 参数
    /// * `field` - 字段名称
    ///
    /// # 返回值
    /// 返回 CSharpReader 配置结构体
    pub fn make_cs_binary_reader(&self, field: &str) -> CSharpReader {
        match self {
            XCellTyped::Vector(v) => CSharpReader { is_vector: true, ..v.get_type().make_cs_binary_reader(field) },
            XCellTyped::List(v) => CSharpReader { is_vector: true, ..v.element_type.make_cs_binary_reader(field) },
            XCellTyped::Map(_) => CSharpReader { is_vector: false, function: "r.ReadString()".to_string(), field: field.to_string() },
            XCellTyped::Optional(v) => v.element_type.make_cs_binary_reader(field),
            _ => CSharpReader { is_vector: false, function: self.as_csharp_reader(), field: field.to_string() },
        }
    }

    /// 返回 C# 二进制读取表达式
    ///
    /// # 返回值
    /// 返回 C# 读取表达式的字符串
    pub fn as_csharp_reader(&self) -> String {
        let str = match self {
            XCellTyped::Boolean(_) => "r.ReadBoolean()",
            XCellTyped::Integer(v) => v.kind.as_csharp_reader(),
            XCellTyped::Decimal(v) => match v.kind {
                DecimalKind::Float32 => "r.ReadSingle()",
                DecimalKind::Float64 => "r.ReadDouble()",
                DecimalKind::Decimal128 => "r.ReadDecimal()",
            },
            XCellTyped::String(_) => "r.ReadString()",
            XCellTyped::Time(_) => "new DateTime(r.ReadInt64(), DateTimeKind.Utc)",
            XCellTyped::Color(_) => "new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
            XCellTyped::Enumerate(v) => return format!("({}) {}", v.name, v.integer.as_csharp_reader()),
            XCellTyped::Array(v) => match v.kind {
                ArrayKind::Vector2 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
                ArrayKind::Vector3 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
                ArrayKind::Vector4 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
                ArrayKind::Quaternion4 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
            },
            XCellTyped::Vector(_) => unreachable!(),
            XCellTyped::Reference(_) => "r.ReadInt32()",
            XCellTyped::List(_) => unreachable!(),
            XCellTyped::Map(_) => unreachable!(),
            XCellTyped::Optional(_) => unreachable!(),
            XCellTyped::Unknown => "null",
        };
        str.to_string()
    }
}

impl BooleanDescription {
    /// 返回布尔类型的 C# 读取语句
    ///
    /// # 参数
    /// * `field` - 字段名称
    ///
    /// # 返回值
    /// 返回 C# 读取语句字符串
    pub fn as_csharp_reader(&self, field: &str) -> String {
        format!("{field} = r.{reader}()", reader = self.as_csharp_reader_function())
    }

    /// 返回布尔类型的 C# 读取函数名称
    ///
    /// # 返回值
    /// 返回读取函数名称字符串
    pub fn as_csharp_reader_function(&self) -> &'static str {
        "ReadBoolean"
    }
}

impl IntegerKind {
    /// 返回整数类型对应的 C# 类型名称
    ///
    /// # 返回值
    /// 返回 C# 类型名称字符串
    pub fn as_csharp_type(&self) -> &'static str {
        match self {
            IntegerKind::Integer8 => "byte",
            IntegerKind::Integer16 => "short",
            IntegerKind::Integer32 => "int",
            IntegerKind::Integer64 => "long",
            IntegerKind::Unsigned8 => "sbyte",
            IntegerKind::Unsigned16 => "ushort",
            IntegerKind::Unsigned32 => "uint",
            IntegerKind::Unsigned64 => "ulong",
        }
    }

    /// 返回整数类型对应的 C# 读取表达式
    ///
    /// # 返回值
    /// 返回 C# 读取表达式字符串
    pub fn as_csharp_reader(&self) -> &'static str {
        match self {
            IntegerKind::Integer8 => "r.ReadByte()",
            IntegerKind::Integer16 => "r.ReadInt16()",
            IntegerKind::Integer32 => "r.ReadInt32()",
            IntegerKind::Integer64 => "r.ReadInt64()",
            IntegerKind::Unsigned8 => "r.ReadSByte()",
            IntegerKind::Unsigned16 => "r.ReadUInt16()",
            IntegerKind::Unsigned32 => "r.ReadUInt32()",
            IntegerKind::Unsigned64 => "r.ReadUInt64()",
        }
    }
}

impl IntegerDescription {}

impl DecimalKind {
    /// 返回小数类型对应的 C# 类型名称
    ///
    /// # 返回值
    /// 返回 C# 类型名称字符串
    pub fn as_csharp_type(&self) -> &'static str {
        match self {
            DecimalKind::Float32 => "float",
            DecimalKind::Float64 => "double",
            DecimalKind::Decimal128 => "decimal",
        }
    }
}

impl DecimalDescription {}

impl StringDescription {}

impl ReferenceDescription {
    /// 返回引用类型对应的 C# 默认值字符串
    pub fn as_csharp_default(&self) -> String {
        match self.default {
            Some(v) => v.to_string(),
            None => "0".to_string(),
        }
    }
}

impl ListDescription {
    /// 返回列表类型对应的 C# 默认值字符串
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_empty() {
            return "new()".to_string();
        }
        format!(
            "new () {{ {} }}",
            self.default.iter().map(|v| v.as_csharp_value()).join(", ")
        )
    }
}
