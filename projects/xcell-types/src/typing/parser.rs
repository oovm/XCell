use super::*;

impl XCellTyped {
    pub fn parse(input: &str, info: &TypeMetaInfo) -> Self {
        let normed = Self::norm_typing(input);
        match normed.as_str() {
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
            "v2" | "vec2" => ArrayDescription::new(ArrayKind::Vector2).into(),
            "v3" | "vec3" => ArrayDescription::new(ArrayKind::Vector3).into(),
            "v4" | "vec4" => ArrayDescription::new(ArrayKind::Vector4).into(),
            "q4" | "quaternion" => ArrayDescription::new(ArrayKind::Quaternion4).into(),
            // slow path
            _ => XCellTyped::parse_complex(input, &normed, info),
        }
    }
    fn parse_complex(raw: &str, normed: &str, info: &TypeMetaInfo) -> Self {
        if info.matches_string(normed) {
            return info.string.clone().into();
        }
        if let Some(s) = info.matches_vector(normed) {
            let typing = XCellTyped::parse(s, info);
            return info.vector.clone().with_type(typing).into();
        }
        EnumerateDescription::new(raw).into()
    }
    fn norm_typing(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for c in input.chars() {
            if c.is_ascii_uppercase() {
                out.push(c.to_ascii_lowercase())
            }
            else if c.is_ascii_whitespace() {
            }
            else {
                out.push(c)
            }
        }
        out
    }
}

impl TypeMetaInfo {
    fn matches_string(&self, s: &str) -> bool {
        self.string.matches_type(s)
    }
    fn matches_vector<'i>(&self, raw: &'i str) -> Option<&'i str> {
        return self.vector.matches_rest(raw);
    }
}
