use super::*;

impl UnityCodegen {
    pub fn write_class(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        tera_render(include_str!("PartClass.cs"), &self.make_context(table), path)?;
        Ok(())
    }
    pub fn write_enum(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        tera_render(include_str!("PartEnum.cs"), &self.make_context(table), path)?;
        Ok(())
    }
    pub fn write_interface(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        tera_render(include_str!("PartShare.cs"), &self.make_context(table), path)?;
        Ok(())
    }
}

impl UnityCodegen {
    fn make_context(&self, table: &XCellTable) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("config", &self);
        ctx.insert("TABLE_NAME", &format!("{}{}", table.class_name(), self.suffix_table));
        ctx.insert("ELEMENT_NAME", &format!("{}{}", table.class_name(), self.suffix_element));
        ctx.insert("ELEMENT_GETTER", &format!("Get{}", self.suffix_element));
        ctx.insert("CLASS_FIELDS", &table.headers.make_class_field());
        ctx
    }
}

#[derive(Serialize)]
struct CsField {
    summary: Vec<String>,
    remarks: Vec<String>,
    writer: Vec<String>,
    typing: String,
    reader: String,
    name: String,
    default: String,
    has_default: bool,
}

impl XCellHeader {
    fn make_class_field(&self) -> CsField {
        let default = self.typing.make_cs_default();
        CsField {
            summary: self.summary.lines().map(|v| v.to_string()).collect(),
            remarks: self.details.lines().map(|v| v.to_string()).collect(),
            has_default: !default.is_empty(),
            typing: self.typing.make_cs_typing(),
            writer: self.typing.make_cs_binary_writer(&self.field_name),
            reader: self.typing.make_cs_binary_reader(),
            name: self.field_name.clone(),
            default,
        }
    }
}

impl XCellHeaders {
    fn make_class_field(&self) -> Vec<CsField> {
        self.inner.iter().map(|v| v.make_class_field()).collect()
    }
}

impl XCellTyped {
    pub fn make_cs_typing(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "bool".to_string(),
            XCellTyped::Integer8(_) => "byte".to_string(),
            XCellTyped::Integer16(_) => "short".to_string(),
            XCellTyped::Integer32(_) => "int".to_string(),
            XCellTyped::Integer64(_) => "long".to_string(),
            XCellTyped::Unsigned8(_) => "sbyte".to_string(),
            XCellTyped::Unsigned16(_) => "ushort".to_string(),
            XCellTyped::Unsigned32(_) => "uint".to_string(),
            XCellTyped::Unsigned64(_) => "ulong".to_string(),
            XCellTyped::Float32(_) => "float".to_string(),
            XCellTyped::Float64(_) => "double".to_string(),
            XCellTyped::Decimal128(_) => "decimal".to_string(),
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => "Color32".to_string(),
            XCellTyped::Custom(v) => v.typing.to_owned(),
        }
    }
    pub fn make_cs_default(&self) -> String {
        match self {
            XCellTyped::Boolean(v) => v.default.to_string(),
            XCellTyped::Integer8(v) => v.default.to_string(),
            XCellTyped::Integer16(v) => v.default.to_string(),
            XCellTyped::Integer32(v) => v.default.to_string(),
            XCellTyped::Integer64(v) => v.default.to_string(),
            XCellTyped::Unsigned8(v) => v.default.to_string(),
            XCellTyped::Unsigned16(v) => v.default.to_string(),
            XCellTyped::Unsigned32(v) => v.default.to_string(),
            XCellTyped::Unsigned64(v) => v.default.to_string(),
            XCellTyped::Float32(v) => v.default.to_string(),
            XCellTyped::Float64(v) => v.default.to_string(),
            XCellTyped::Decimal128(v) => v.default.to_string(),
            XCellTyped::String(v) => format!("{:?}", v.default),
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(v) => v.make_cs_color32(),
            XCellTyped::Custom(v) => v.default.to_string(),
        }
    }
    fn make_cs_binary_writer(&self, field: &str) -> Vec<String> {
        let mut out = vec![];
        match self {
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                out.push(format!("w.Write({field}.r);"));
                out.push(format!("w.Write({field}.g);"));
                out.push(format!("w.Write({field}.b);"));
                out.push(format!("w.Write({field}.a);"));
            }
            XCellTyped::Custom(v) => out.push(v.default.to_string()),
            _ => {
                out.push(format!("w.Write({field});"));
            }
        }
        out
    }
    fn make_cs_binary_reader(&self) -> String {
        match self {
            XCellTyped::Boolean(_) => "r.ReadBoolean()".to_string(),
            XCellTyped::Integer8(_) => "r.ReadByte()".to_string(),
            XCellTyped::Integer16(_) => "r.ReadInt16()".to_string(),
            XCellTyped::Integer32(_) => "r.ReadInt32()".to_string(),
            XCellTyped::Integer64(_) => "r.ReadInt64()".to_string(),
            XCellTyped::Unsigned8(_) => "r.ReadSByte()".to_string(),
            XCellTyped::Unsigned16(_) => "r.ReadUInt16()".to_string(),
            XCellTyped::Unsigned32(_) => "r.ReadUInt32()".to_string(),
            XCellTyped::Unsigned64(_) => "r.ReadUInt64()".to_string(),
            XCellTyped::Float32(_) => "r.ReadSingle()".to_string(),
            XCellTyped::Float64(_) => "r.ReadDouble()".to_string(),
            XCellTyped::Decimal128(_) => "r.ReadDecimal()".to_string(),
            XCellTyped::String(_) => "r.ReadString()".to_string(),

            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => "new Color32(r.ReadByte(), r.ReadByte(), r.ReadByte(), r.ReadByte())".to_string(),
            XCellTyped::Custom(v) => v.default.to_string(),
        }
    }
}

impl ColorDescription {
    pub fn make_cs_color32(&self) -> String {
        let [r, g, b, a] = self.default.to_rgba8();
        format!("new Color32({r}, {g}, {b}, {a})")
    }
}
