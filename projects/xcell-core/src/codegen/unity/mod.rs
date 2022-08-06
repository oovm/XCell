use super::*;

impl UnityCodegen {
    pub fn write_class(&self, table: &XCellTable, root: &Path) -> XResult<()> {
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = self.unity_csharp_path(root, &file)?;
        log::info!("写入 {}", self.unity_relative(&file));
        tera_render(include_str!("PartClass.cs"), &self.make_context(table), &path)?;
        Ok(())
    }
    pub fn write_manager(&self, table: &TableMerged, root: &Path) -> XResult<()> {
        let path = self.unity_manager_path(root)?;
        tera_render(include_str!("PartManager.cs"), &self.make_manager(table), &path)?;
        Ok(())
    }
}

impl UnityCodegen {
    fn make_context(&self, table: &XCellTable) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("config", &self);
        ctx.insert("CLASS_NAME", &table.name);
        ctx.insert("TABLE_NAME", &format!("{}{}", table.name, self.suffix_table));
        ctx.insert("ELEMENT_NAME", &format!("{}{}", table.name, self.suffix_element));
        ctx.insert("ELEMENT_GETTER", &format!("Get{}", self.suffix_element));
        ctx.insert("KEY", &table.data.key_field());
        ctx.insert("ID_TYPE", &table.data.key_type());
        let is_enum = table.is_enumerate();
        ctx.insert("enumerate", &is_enum);
        // ctx.insert("CLASS_FIELDS", &table.data.make_class_field(is_enum));
        ctx
    }
    fn make_manager(&self, table: &TableMerged) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("config", &self);
        #[derive(Serialize)]
        struct CsTable {
            name: String,
            private: String,
        }
        ctx.insert(
            "tables",
            &table
                .table_names()
                .into_iter()
                .map(|name| {
                    let name = format!("{}{}", name, self.suffix_table);
                    let private = format!("_{}", name.to_case(Case::Snake));
                    CsTable { name, private }
                })
                .collect_vec(),
        );

        ctx
    }
}

#[derive(Serialize)]
struct CSharpField {
    summary: Vec<String>,
    remarks: Vec<String>,

    typing: String,
    reader: CSharpReader,
    writer: CSharpWriter,
    name: String,
    getter: String,
    default: String,
    has_default: bool,
}

impl XCellHeader {
    fn make_class_field(&self, _is_enum: bool) -> CSharpField {
        let default = self.typing.as_csharp_default();
        CSharpField {
            summary: self.summary.lines().map(|v| v.to_string()).collect(),
            remarks: self.details.lines().map(|v| v.to_string()).collect(),
            has_default: !default.is_empty(),
            typing: self.typing.as_csharp_type(),
            writer: self.typing.make_cs_binary_writer(&self.field_name),
            reader: self.typing.make_cs_binary_reader(&self.field_name),
            name: self.field_name.clone(),
            getter: format!("Get{}", self.field_name.to_case(Case::Pascal)),
            default,
        }
    }
}
