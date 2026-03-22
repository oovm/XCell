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
    /// 向量类型 `Vec<T>`
    Vec {
        /// 元素类型
        element: Box<TypeExpr>,
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
    /// 命名类型（自定义类型名，如枚举、结构体）
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
    /// 32位浮点数
    F32,
    /// 64位浮点数
    F64,
    /// 字符串
    String,
    /// UTF-8 字符
    Utf8,
    /// UTF-16 字符
    Utf16,
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
}

impl PrimitiveType {
    /// 从字符串解析原始类型
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bool" | "boolean" => Some(Self::Bool),
            "i8" | "char" => Some(Self::I8),
            "i16" | "short" => Some(Self::I16),
            "i32" | "int" => Some(Self::I32),
            "i64" | "long" | "longlong" => Some(Self::I64),
            "u8" | "byte" => Some(Self::U8),
            "u16" | "ushort" => Some(Self::U16),
            "u32" | "uint" => Some(Self::U32),
            "u64" | "ulong" => Some(Self::U64),
            "f32" | "float" => Some(Self::F32),
            "f64" | "double" => Some(Self::F64),
            "string" | "str" | "text" => Some(Self::String),
            "utf8" => Some(Self::Utf8),
            "utf16" => Some(Self::Utf16),
            "color" | "colour" => Some(Self::Color),
            "time" => Some(Self::Time),
            "datetime" => Some(Self::DateTime),
            "date" => Some(Self::Date),
            "vec2" | "v2" => Some(Self::Vec2),
            "vec3" | "v3" => Some(Self::Vec3),
            "vec4" | "v4" => Some(Self::Vec4),
            _ => None,
        }
    }
}

/// 字段约束
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldConstraint {
    /// 唯一约束 `@field_name`
    Unique,
    /// 主键约束 `@@field_name`
    Primary,
}

/// 字段表达式（字段名 + 可选约束）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldExpr {
    /// 字段名
    pub name: String,
    /// 约束（如果有）
    pub constraint: Option<FieldConstraint>,
}

/// 表类型（元数据）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableKind {
    /// 字典表 `@dict`
    Dict,
    /// 类表 `@class`
    Class,
    /// 枚举表 `@enum`
    Enum,
    /// 语言表 `@lang`
    Lang,
    /// 配置表 `@config`
    Config,
}

/// 元数据表达式（第一行第一个单元格）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaExpr {
    /// 表类型
    pub kind: TableKind,
    /// 复合唯一约束字段列表
    pub unique_fields: Vec<String>,
}

/// 字段元属性（写在 Excel 注释中）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldMeta {
    /// 主键 `@primary`
    Primary,
    /// 默认值 `@default(value)`
    Default(String),
    /// 虚拟字段 `@virtual`
    Virtual,
    /// 计算属性 `@computed`
    Computed,
}

/// 类型元属性（写在 Excel 注释中）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeMeta {
    /// 最小值 `@min(value)`
    Min(i64),
    /// 最大值 `@max(value)`
    Max(i64),
    /// 范围 `@range(min, max)`
    Range(i64, i64),
    /// 默认值 `@default(value)`
    Default(String),
}

impl std::fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeExpr::Primitive(p) => write!(f, "{}", p),
            TypeExpr::Reference { target } => write!(f, "&{}", target),
            TypeExpr::List { element } => write!(f, "[{}]", element),
            TypeExpr::FixedArray { element, length } => write!(f, "[{}; {}]", element, length),
            TypeExpr::Vec { element } => write!(f, "Vec<{}>", element),
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
            PrimitiveType::F32 => write!(f, "f32"),
            PrimitiveType::F64 => write!(f, "f64"),
            PrimitiveType::String => write!(f, "string"),
            PrimitiveType::Utf8 => write!(f, "utf8"),
            PrimitiveType::Utf16 => write!(f, "utf16"),
            PrimitiveType::Color => write!(f, "color"),
            PrimitiveType::Time => write!(f, "time"),
            PrimitiveType::DateTime => write!(f, "datetime"),
            PrimitiveType::Date => write!(f, "date"),
            PrimitiveType::Vec2 => write!(f, "vec2"),
            PrimitiveType::Vec3 => write!(f, "vec3"),
            PrimitiveType::Vec4 => write!(f, "vec4"),
        }
    }
}

impl std::fmt::Display for FieldExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.constraint {
            Some(FieldConstraint::Unique) => write!(f, "@{}", self.name),
            Some(FieldConstraint::Primary) => write!(f, "@@{}", self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

impl std::fmt::Display for TableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableKind::Dict => write!(f, "@dict"),
            TableKind::Class => write!(f, "@class"),
            TableKind::Enum => write!(f, "@enum"),
            TableKind::Lang => write!(f, "@lang"),
            TableKind::Config => write!(f, "@config"),
        }
    }
}

impl std::fmt::Display for MetaExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;
        if !self.unique_fields.is_empty() {
            write!(f, " @unique(")?;
            for (i, field) in self.unique_fields.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", field)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for FieldMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldMeta::Primary => write!(f, "@primary"),
            FieldMeta::Default(v) => write!(f, "@default({})", v),
            FieldMeta::Virtual => write!(f, "@virtual"),
            FieldMeta::Computed => write!(f, "@computed"),
        }
    }
}

impl std::fmt::Display for TypeMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeMeta::Min(v) => write!(f, "@min({})", v),
            TypeMeta::Max(v) => write!(f, "@max({})", v),
            TypeMeta::Range(min, max) => write!(f, "@range({}, {})", min, max),
            TypeMeta::Default(v) => write!(f, "@default({})", v),
        }
    }
}
