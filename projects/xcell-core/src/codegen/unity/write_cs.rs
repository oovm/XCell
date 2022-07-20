use super::*;

impl TypeMetaInfo {
    pub fn write_class(&self, table: &XCellTable) -> XResult<()> {
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = table.path.with_file_name(file).with_extension("cs");
        tera_render(include_str!("PartClass.cs"), &self.make_context(table), &path)?;
        Ok(())
    }
    pub fn write_interface(&self, table: &XCellTable, path: &Path) -> XResult<()> {
        tera_render(include_str!("PartInterface.cs"), &self.make_context(table), path)?;
        Ok(())
    }
}

impl TypeMetaInfo {
    fn make_context(&self, table: &XCellTable) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("config", &self);
        ctx.insert("CLASS_NAME", &table.name);
        ctx.insert("TABLE_NAME", &format!("{}{}", table.name, self.suffix_table));
        ctx.insert("ELEMENT_NAME", &format!("{}{}", table.name, self.suffix_element));
        ctx.insert("ELEMENT_GETTER", &format!("Get{}", self.suffix_element));
        ctx.insert("ID_TYPE", &table.headers.key_type());
        let is_enum = table.is_enumerate();
        ctx.insert("enumerate", &is_enum);
        ctx.insert("CLASS_FIELDS", &table.headers.make_class_field(is_enum));
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
    getter: String,
    default: String,
    has_default: bool,
}

impl XCellHeaders {
    fn make_class_field(&self, is_enum: bool) -> Vec<CsField> {
        let mut items = self.inner.iter();
        if is_enum {
            items.next();
        }
        items.map(|v| v.make_class_field(is_enum)).collect()
    }
    fn key_type(&self) -> String {
        match self.inner.first() {
            Some(s) => match &s.typing {
                XCellTyped::Enumerate(e) => e.integer.as_csharp_type().to_string(),
                _ => s.typing.as_csharp_type(),
            },
            None => "int".to_string(),
        }
    }
}

impl XCellHeader {
    fn make_class_field(&self, _is_enum: bool) -> CsField {
        let default = self.typing.as_csharp_default();
        CsField {
            summary: self.summary.lines().map(|v| v.to_string()).collect(),
            remarks: self.details.lines().map(|v| v.to_string()).collect(),
            has_default: !default.is_empty(),
            typing: self.typing.as_csharp_type(),
            writer: self.typing.make_cs_binary_writer(&self.field_name),
            reader: self.typing.make_cs_binary_reader(),
            name: self.field_name.clone(),
            getter: format!("Get{}", self.field_name.to_case(Case::Pascal)),
            default,
        }
    }
}
