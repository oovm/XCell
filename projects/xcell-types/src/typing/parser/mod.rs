// impl FromStr for XCellTyped {
//     type Err = XError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let out = match s.to_ascii_lowercase().as_str() {
//             "str" | "string" => Self::String(Default::default()),
//             "language" | "languagestring" | "languageid" => Self::String(Default::default()),
//             "bool" | "boolean" => Self::Boolean(Default::default()),
//             // int
//             "byte" | "i8" => Self::Integer(IntegerDescription::range(i8::MIN, i8::MAX, IntegerKind::Integer8)),
//             "short" | "i16" => Self::Integer(IntegerDescription::range(i16::MIN, i16::MAX, IntegerKind::Integer16)),
//             "int" | "i32" => Self::Integer(IntegerDescription::range(i32::MIN, i32::MAX, IntegerKind::Integer32)),
//             "long" | "i64" => Self::Integer(IntegerDescription::range(i64::MIN, i64::MAX, IntegerKind::Integer64)),
//             // unsigned
//             "sbyte" | "u8" => Self::Integer(IntegerDescription::range(u8::MIN, u8::MAX, IntegerKind::Unsigned8)),
//             "ushort" | "u16" => Self::Integer(IntegerDescription::range(u16::MIN, u16::MAX, IntegerKind::Unsigned16)),
//             "uint" | "u32" => Self::Integer(IntegerDescription::range(u32::MIN, u32::MAX, IntegerKind::Unsigned32)),
//             "ulong" | "u64" => Self::Integer(IntegerDescription::range(u64::MIN, u64::MAX, IntegerKind::Unsigned64)),
//             // float
//             "float" | "f32" => Self::Decimal(Default::default()),
//             "double" | "f64" => Self::Float64(Default::default()),
//             "decimal" | "d128" | "f128" => Self::Decimal128(Default::default()),
//             // other
//             "color" | "color32" => Self::Color(Default::default()),
//             "date" | "time" | "datetime" => Self::Time(Default::default()),
//             // array
//             "v2" | "vec2" => Self::Custom(ArrayDescription::new(s)),
//             "v3" | "vec3" => Self::Custom(ArrayDescription::new(s)),
//             "v4" | "vec4" => Self::Custom(ArrayDescription::new(s)),
//             "q4" | "quaternion" => Self::Custom(ArrayDescription::new(s)),
//             "c4" | "color4" => Self::Custom(ArrayDescription::new(s)),
//             // enum
//             "enum" => Self::Enumerate(Default::default()),
//             _ => Self::Custom(CustomDescription::new(s)),
//         };
//         Ok(out)
//     }
// }
