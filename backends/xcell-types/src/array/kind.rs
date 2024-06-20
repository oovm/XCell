use super::*;

impl Default for ArrayKind {
    fn default() -> Self {
        Self::Vector3
    }
}
impl Deref for ArrayDescription {
    type Target = ArrayKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl ArrayKind {
    pub fn as_csharp_type(&self) -> &'static str {
        match self {
            ArrayKind::Vector2 => "Vector2",
            ArrayKind::Vector3 => "Vector3",
            ArrayKind::Vector4 => "Vector4",
            ArrayKind::Quaternion4 => "Quaternion4",
        }
    }

    #[cfg(feature = "typescript")]
    /// 返回当前数组类型对应的 TypeScript 类型名称
    pub fn as_typescript_type(&self) -> &'static str {
        match self {
            ArrayKind::Vector2 => "[number, number]",
            ArrayKind::Vector3 => "[number, number, number]",
            ArrayKind::Vector4 => "[number, number, number, number]",
            ArrayKind::Quaternion4 => "[number, number, number, number]",
        }
    }
    // pub fn as_csharp_reader(&self) -> &'static str {
    //     "ReadSingle"
    // }
}
