use super::*;

impl XCellTyped {
    pub fn parse(input: &str) -> XResult<Self> {
        let typing = input.to_ascii_lowercase();
        let out: XCellTyped = match typing.trim() {
            "str" | "string" => StringDescription::default().into(),
            "language" | "languagestring" | "languageid" => StringDescription::default().into(),
            "bool" | "boolean" => BooleanDescription::new(false).into(),
            // int
            "byte" | "i8" => IntegerDescription::range(i8::MIN, i8::MAX, IntegerKind::Integer8).into(),
            "short" | "i16" => IntegerDescription::range(i16::MIN, i16::MAX, IntegerKind::Integer16).into(),
            "int" | "i32" => IntegerDescription::range(i32::MIN, i32::MAX, IntegerKind::Integer32).into(),
            "long" | "i64" => IntegerDescription::range(i64::MIN, i64::MAX, IntegerKind::Integer64).into(),
            // unsigned
            "sbyte" | "u8" => IntegerDescription::range(u8::MIN, u8::MAX, IntegerKind::Unsigned8).into(),
            "ushort" | "u16" => IntegerDescription::range(u16::MIN, u16::MAX, IntegerKind::Unsigned16).into(),
            "uint" | "u32" => IntegerDescription::range(u32::MIN, u32::MAX, IntegerKind::Unsigned32).into(),
            "ulong" | "u64" => IntegerDescription::range(u64::MIN, u64::MAX, IntegerKind::Unsigned64).into(),
            // float
            "float" | "f32" => Self::Decimal(Default::default()),
            "double" | "f64" => Self::Decimal(Default::default()),
            "decimal" | "d128" | "f128" => Self::Decimal(Default::default()),
            // other
            "color" | "colour" => Self::Color(Default::default()),
            // "c4" | "color32" | "color4" => Self::Custom(ArrayDescription::new(s)),
            // "date" | "time" | "datetime" => Self::Time(Default::default()),
            // array
            // "v2" | "vec2" => Self::Custom(ArrayDescription::new(s)),
            "v3" | "vec3" => ArrayDescription::new(ArrayKind::Vector3).into(),
            // "v4" | "vec4" => Self::Custom(ArrayDescription::new(s)),
            // "q4" | "quaternion" => Self::Custom(ArrayDescription::new(s)),

            // enum
            "enum" => Self::Enumerate(Default::default()),
            _ => CustomDescription::new(typing).into(),
        };
        Ok(out)
    }
}

impl Default for XTableKind {
    fn default() -> Self {
        Self::SortedMap
    }
}

impl FromStr for XTableKind {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let kind = match input.to_ascii_lowercase().trim() {
            "enum" => XTableKind::Enumerate,
            _ => XTableKind::SortedMap,
        };
        Ok(kind)
    }
}

impl XTableKind {
    pub fn new<S>(input: S) -> Self
    where
        S: AsRef<str>,
    {
        unsafe { XTableKind::from_str(input.as_ref()).unwrap_unchecked() }
    }
}
