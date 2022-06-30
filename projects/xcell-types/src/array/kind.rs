
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
            ArrayKind::Color4 => "Color4",
            ArrayKind::Quaternion4 => "Quaternion4",
        }
    }
    // pub fn as_csharp_reader(&self) -> &'static str {
    //     "ReadSingle"
    // }
}
