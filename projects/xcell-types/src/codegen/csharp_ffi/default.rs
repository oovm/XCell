use super::*;

impl XCellTyped {
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
        }
    }
}

impl XCellValue {
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
            XCellValue::Float32(_) => {
                todo!()
            }
            XCellValue::Float64(_) => {
                todo!()
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
            XCellValue::String(_) => {
                todo!()
            }
            XCellValue::Color(_) => {
                todo!()
            }
            XCellValue::Custom(_) => {
                todo!()
            }
            XCellValue::Vector(_) => {
                todo!()
            }
        }
    }
}

impl BooleanDescription {
    pub fn as_csharp_default(&self) -> String {
        match self.default {
            true => "true".to_string(),
            false => "".to_string(),
        }
    }
}

impl IntegerDescription {
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_zero() { "".to_string() } else { self.default.to_string() }
    }
}

impl DecimalDescription {
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_zero() { "".to_string() } else { self.default.to_string() }
    }
}

impl StringDescription {
    pub fn as_csharp_default(&self) -> String {
        if self.default.is_empty() { "".to_string() } else { format!("{:?}", self.default) }
    }
}

impl TimeDescription {
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
            None => {
                format!("new DateTime()")
            }
        }
    }
}

impl ColorDescription {
    fn as_csharp_default(&self) -> String {
        let [r, g, b, a] = self.default.to_rgba8();
        format!("new Color32({r}, {g}, {b}, {a})")
    }
}

impl ArrayDescription {
    fn as_csharp_default(&self) -> String {
        // match self.kind {
        //     ArrayKind::Vector2 => {}
        //     ArrayKind::Vector3 => {}
        //     ArrayKind::Vector4 => {}
        //     ArrayKind::Color4 => {}
        //     ArrayKind::Quaternion4 => {}
        // }
        //
        // let [r, g, b, a] = self.default.to_rgba8();
        format!("new ()")
    }
}

impl XCellTyped {
    pub fn make_cs_binary_writer(&self, field: &str) -> CSharpWriter {
        let properties = match self {
            XCellTyped::Time(_) => vec![".Ticks".to_string()],
            XCellTyped::Color(_) => vec![".r".to_string(), ".g".to_string(), ".b".to_string(), ".a".to_string()],
            // XCellTyped::Enumerate(v) => out.push(v.default.to_string()),
            XCellTyped::Enumerate(_) => {
                return CSharpWriter {
                    is_vector: false,
                    field: field.to_string(),
                    cast: "(int) ".to_string(),
                    properties: vec!["".to_string()],
                };
            }
            XCellTyped::Vector(v) => return CSharpWriter { is_vector: true, ..v.typing.make_cs_binary_writer(field) },
            _ => vec!["".to_string()],
        };
        CSharpWriter { is_vector: false, field: field.to_string(), cast: "".to_string(), properties }
    }
    pub fn make_cs_binary_reader(&self, field: &str) -> CSharpReader {
        match self {
            XCellTyped::Vector(v) => CSharpReader { is_vector: true, ..v.typing.make_cs_binary_reader(field) },
            _ => CSharpReader { is_vector: false, function: self.as_csharp_reader(), field: field.to_string() },
        }
    }
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
            XCellTyped::Enumerate(v) => return format!("({}) {}", v.typing, v.integer.as_csharp_reader()),
            XCellTyped::Array(v) => match v.kind {
                ArrayKind::Vector2 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
                ArrayKind::Vector3 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
                ArrayKind::Vector4 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
                ArrayKind::Color4 => unreachable!(),
                ArrayKind::Quaternion4 => "new Vector2(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())",
            },
            XCellTyped::Vector(_) => unreachable!(),
        };
        str.to_string()
    }
}

impl BooleanDescription {
    pub fn as_csharp_reader(&self, field: &str) -> String {
        format!("{field} = r.{reader}()", reader = self.as_csharp_reader_function())
    }
    pub fn as_csharp_reader_function(&self) -> &'static str {
        "ReadBoolean"
    }
}

impl IntegerKind {
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
