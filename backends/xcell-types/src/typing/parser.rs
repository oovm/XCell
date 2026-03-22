use super::*;

impl XCellTyped {
    pub fn parse(input: &str, info: &TypeMetaInfo) -> Self {
        let trimmed = input.trim();
        let normed = Self::norm_typing(trimmed);
        match normed.as_str() {
            "bool" | "boolean" => info.boolean.clone().into(),
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
            "color32" => Self::Color(Default::default()),
            // "date" | "time" | "datetime" => Self::Time(Default::default()),
            // array
            "v2" | "vec2" => ArrayDescription::new(ArrayKind::Vector2).into(),
            "v3" | "vec3" => ArrayDescription::new(ArrayKind::Vector3).into(),
            "v4" | "vec4" => ArrayDescription::new(ArrayKind::Vector4).into(),
            "q4" | "quaternion" => ArrayDescription::new(ArrayKind::Quaternion4).into(),
            // slow path
            _ => XCellTyped::parse_complex(trimmed, &normed, info),
        }
    }
    fn parse_complex(raw: &str, normed: &str, info: &TypeMetaInfo) -> Self {
        // 检查引用类型: &TableName
        if raw.starts_with('&') {
            let table_name = raw[1..].trim();
            if !table_name.is_empty() {
                return ReferenceDescription::new(table_name).into();
            }
        }
        // 检查引用类型: ref<TableName> 或 Ref<TableName>
        if normed.starts_with("ref<") && normed.ends_with('>') {
            let table_name = &normed[4..normed.len() - 1];
            if !table_name.is_empty() {
                return ReferenceDescription::new(table_name).into();
            }
        }
        // 检查列表类型: [T] 或 [T; N]
        if raw.starts_with('[') && raw.ends_with(']') {
            return Self::parse_list_type(raw, info);
        }
        if info.matches_string(normed) {
            return info.string.clone().into();
        }
        if let Some(s) = info.matches_vector(raw) {
            let typing = XCellTyped::parse(s, info);
            return info.vector.clone().with_type(typing).into();
        }
        EnumerateDescription::new(raw).into()
    }

    /// 解析列表类型语法
    ///
    /// 支持格式:
    /// - `[T]` - 动态数组
    /// - `[T; N]` - 固定长度数组
    fn parse_list_type(raw: &str, info: &TypeMetaInfo) -> Self {
        let inner = &raw[1..raw.len() - 1];
        let inner = inner.trim();
        // 检查是否有固定长度: [T; N]
        if let Some(semi_pos) = inner.find(';') {
            let type_part = inner[..semi_pos].trim();
            let len_part = inner[semi_pos + 1..].trim();
            if let Ok(fixed_length) = len_part.parse::<usize>() {
                let element_type = XCellTyped::parse(type_part, info);
                let list = ListDescription { element_type, fixed_length: Some(fixed_length), ..Default::default() };
                return list.into();
            }
        }
        // 动态数组: [T]
        let element_type = XCellTyped::parse(inner, info);
        let list = ListDescription { element_type, ..Default::default() };
        list.into()
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
