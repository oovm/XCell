use super::*;

impl Default for DecimalKind {
    fn default() -> Self {
        DecimalKind::Float32
    }
}

#[cfg(feature = "typescript")]
impl DecimalKind {
    /// 返回当前小数类型对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> &'static str {
        match self {
            DecimalKind::Float32 => "number",
            DecimalKind::Float64 => "number",
            DecimalKind::Decimal128 => "number",
        }
    }
}
