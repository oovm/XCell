use serde::{Deserialize, Serialize};

/// 类型表达式 AST
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpr {
    /// 原始类型
    Primitive(PrimitiveType),
    /// 引用类型 `&TableName`
    Reference {
        /// 目标表名
        target: String,
    },
    /// Ref 类型 `ref<TableName>`
    Ref {
        /// 目标表名
        target: String,
    },
    /// 列表类型 `[T]`
    List {
        /// 元素类型
        element: Box<TypeExpr>,
    },
    /// 固定长度数组 `[T; N]`
    FixedArray {
        /// 元素类型
        element: Box<TypeExpr>,
        /// 长度
        length: usize,
    },
    /// 泛型类型 `Name<T1, T2, ...>`
    Generic {
        /// 类型名
        name: String,
        /// 类型参数
        args: Vec<TypeExpr>,
    },
    /// 元组类型 `(T1, T2, ...)`
    Tuple(Vec<TypeExpr>),
    /// 可选类型 `T?`
    Optional(Box<TypeExpr>),
    /// 指针类型 `*T`
    Pointer(Box<TypeExpr>),
    /// 独一类型 `@T` 或 `@@T`
    Unique {
        /// 内部类型
        inner: Box<TypeExpr>,
        /// 是否为主键
        is_primary: bool,
    },
    /// 命名类型（自定义类型名）
    Named(String),
}

/// 原始类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    /// 布尔类型
    Bool,
    /// 8位有符号整数
    I8,
    /// 16位有符号整数
    I16,
    /// 32位有符号整数
    I32,
    /// 64位有符号整数
    I64,
    /// 8位无符号整数
    U8,
    /// 16位无符号整数
    U16,
    /// 32位无符号整数
    U32,
    /// 64位无符号整数
    U64,
    /// 128位有符号整数
    I128,
    /// 128位无符号整数
    U128,
    /// 32位浮点数
    F32,
    /// 64位浮点数
    F64,
    /// 字符串
    String,
    /// 字符
    Char,
    /// 颜色
    Color,
    /// 时间
    Time,
    /// 日期时间
    DateTime,
    /// 日期
    Date,
    /// 2D向量
    Vec2,
    /// 3D向量
    Vec3,
    /// 4D向量
    Vec4,
    /// 四元数
    Quaternion,
}

impl PrimitiveType {
    /// 从字符串解析原始类型
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bool" | "boolean" => Some(Self::Bool),
            "i8" | "byte" => Some(Self::I8),
            "i16" | "short" => Some(Self::I16),
            "i32" | "int" => Some(Self::I32),
            "i64" | "long" => Some(Self::I64),
            "u8" | "sbyte" => Some(Self::U8),
            "u16" | "ushort" => Some(Self::U16),
            "u32" | "uint" => Some(Self::U32),
            "u64" | "ulong" => Some(Self::U64),
            "i128" => Some(Self::I128),
            "u128" => Some(Self::U128),
            "f32" | "float" => Some(Self::F32),
            "f64" | "double" => Some(Self::F64),
            "string" | "str" | "text" => Some(Self::String),
            "char" | "utf8" => Some(Self::Char),
            "color" | "colour" => Some(Self::Color),
            "time" => Some(Self::Time),
            "datetime" => Some(Self::DateTime),
            "date" => Some(Self::Date),
            "vec2" | "v2" => Some(Self::Vec2),
            "vec3" | "v3" => Some(Self::Vec3),
            "vec4" | "v4" => Some(Self::Vec4),
            "quat" | "quaternion" | "q4" => Some(Self::Quaternion),
            _ => None,
        }
    }
}

/// 类型修饰符
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeModifier {
    /// 可空修饰符 `?`
    Optional,
    /// 引用修饰符 `&`
    Reference,
    /// 指针修饰符 `*`
    Pointer,
    /// 独一修饰符 `@`
    Unique,
    /// 主键修饰符 `@@`
    PrimaryKey,
}

impl std::fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeExpr::Primitive(p) => write!(f, "{:?}", p),
            TypeExpr::Reference { target } => write!(f, "&{}", target),
            TypeExpr::Ref { target } => write!(f, "ref<{}>", target),
            TypeExpr::List { element } => write!(f, "[{}]", element),
            TypeExpr::FixedArray { element, length } => write!(f, "[{}; {}]", element, length),
            TypeExpr::Generic { name, args } => {
                write!(f, "{}<", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ">")
            }
            TypeExpr::Tuple(elements) => {
                write!(f, "(")?;
                for (i, e) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            TypeExpr::Optional(inner) => write!(f, "{}?", inner),
            TypeExpr::Pointer(inner) => write!(f, "*{}", inner),
            TypeExpr::Unique { inner, is_primary } => {
                if *is_primary {
                    write!(f, "@@{}", inner)
                } else {
                    write!(f, "@{}", inner)
                }
            }
            TypeExpr::Named(name) => write!(f, "{}", name),
        }
    }
}

impl std::fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::I8 => write!(f, "i8"),
            PrimitiveType::I16 => write!(f, "i16"),
            PrimitiveType::I32 => write!(f, "i32"),
            PrimitiveType::I64 => write!(f, "i64"),
            PrimitiveType::U8 => write!(f, "u8"),
            PrimitiveType::U16 => write!(f, "u16"),
            PrimitiveType::U32 => write!(f, "u32"),
            PrimitiveType::U64 => write!(f, "u64"),
            PrimitiveType::I128 => write!(f, "i128"),
            PrimitiveType::U128 => write!(f, "u128"),
            PrimitiveType::F32 => write!(f, "f32"),
            PrimitiveType::F64 => write!(f, "f64"),
            PrimitiveType::String => write!(f, "string"),
            PrimitiveType::Char => write!(f, "char"),
            PrimitiveType::Color => write!(f, "color"),
            PrimitiveType::Time => write!(f, "time"),
            PrimitiveType::DateTime => write!(f, "datetime"),
            PrimitiveType::Date => write!(f, "date"),
            PrimitiveType::Vec2 => write!(f, "vec2"),
            PrimitiveType::Vec3 => write!(f, "vec3"),
            PrimitiveType::Vec4 => write!(f, "vec4"),
            PrimitiveType::Quaternion => write!(f, "quaternion"),
        }
    }
}
