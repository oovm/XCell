use serde::Serialize;
use tera::Context;

use crate::XCellHeaders;

use super::*;

impl UnityCodegen {
    pub fn write_class(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let mut tera = Tera::default();
        tera.autoescape_on(vec![]);
        tera.add_raw_template("T", include_str!("PartClass.cs")).unwrap();
        let out = tera.render("T", &self.make_context(table)).unwrap();
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    pub fn write_enum(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let mut tera = Tera::default();
        tera.autoescape_on(vec![]);
        tera.add_raw_template("T", include_str!("PartEnum.cs")).unwrap();
        let out = tera.render("T", &self.make_context(table)).unwrap();
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    pub fn write_interface(&self, table: &XCellTable, path: &Path) -> Result<(), XError> {
        let mut file = File::create(path)?;
        let mut tera = Tera::default();
        tera.autoescape_on(vec![]);
        tera.add_raw_template("T", include_str!("PartShare.cs")).unwrap();
        let out = tera.render("T", &self.make_context(table)).unwrap();
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

impl UnityCodegen {
    fn make_context(&self, table: &XCellTable) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("SUPPORT_BINARY", &self.support_binary);
        ctx.insert("SUPPORT_CLONE", &self.support_clone);
        ctx.insert("NAMESPACE", &self.namespace.join("."));
        ctx.insert("TABLE_NAME", &format!("{}{}", table.class_name(), self.table_suffix));
        ctx.insert("ELEMENT_NAME", &format!("{}{}", table.class_name(), self.element_suffix));
        ctx.insert("ELEMENT_GETTER", &format!("Get{}", self.element_suffix));
        ctx.insert("CLASS_FIELDS", &table.headers.make_class_field());
        ctx
    }

    fn write_namespace(&self, f: &mut impl Write, template: &str) -> std::io::Result<usize> {
        let ns = self.namespace.join(".");
        let define = template.replace("__NAMESPACE__", &ns);
        f.write(define.as_bytes())
    }
}

#[derive(Serialize)]
struct CsField {
    summary: Vec<String>,
    remarks: Vec<String>,
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
            XCellTyped::Decimal128(_) => {
                todo!()
            }
            XCellTyped::String(_) => "string".to_string(),
            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                todo!()
            }
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
            XCellTyped::Decimal128(_) => {
                todo!()
            }
            XCellTyped::String(v) => format!("{:?}", v.default),

            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                todo!()
            }
            XCellTyped::Custom(v) => v.default.to_string(),
        }
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
            XCellTyped::Decimal128(_) => {
                todo!()
            }
            XCellTyped::String(_) => "r.ReadString()".to_string(),

            XCellTyped::Datetime(_) => {
                todo!()
            }
            XCellTyped::Color(_) => {
                todo!()
            }
            XCellTyped::Custom(v) => v.default.to_string(),
        }
    }
}
