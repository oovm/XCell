use super::*;

impl Default for DecimalKind {
    fn default() -> Self {
        DecimalKind::Float32
    }
}

impl Deref for DecimalDescription {
    type Target = DecimalKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl DecimalKind {
    pub fn as_csharp_type(&self) -> &'static str {
        match self {
            DecimalKind::Float32 => "float",
            DecimalKind::Float64 => "double",
            DecimalKind::Decimal128 => "decimal",
        }
    }
    pub fn as_csharp_reader(&self) -> &'static str {
        match self {
            DecimalKind::Float32 => "ReadSingle",
            DecimalKind::Float64 => "ReadDouble",
            DecimalKind::Decimal128 => "ReadDecimal",
        }
    }
}
