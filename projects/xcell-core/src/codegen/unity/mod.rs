use super::*;

impl UnityCodegen {
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if let Some(s) = self.unity_csharp_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        if let Some(s) = self.unity_binary_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        Ok(())
    }
    pub fn write_manager(&self, table: &TableMerged, root: &Path) -> XResult<()> {
        let path = self.unity_manager_path(root)?;
        tera_render(include_str!("PartManager.cs"), &self.make_manager(table), &path)?;
        Ok(())
    }
    pub fn write_class(&self, table: &XCellTable, root: &Path) -> XResult<()> {
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = self.unity_csharp_path(root, &file)?;
        log::info!("写入 {}", self.unity_cs_relative(&file));
        tera_render(include_str!("PartClass.cs"), &self.make_context(table), &path)?;
        Ok(())
    }
    pub fn write_binary(&self, table: &XCellTable, root: &Path) -> XResult<()> {
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = self.unity_binary_path(root, &file)?;
        log::info!("写入 {}", self.unity_bin_relative(&file));
        let cg = BinaryCodegen {};
        cg.write_binary(table, &path)?;
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
        ctx.insert("ID_TYPE", &table.data.key_type().as_csharp_type());
        let is_enum = table.is_enumerate();
        ctx.insert("enumerate", &is_enum);
        ctx.insert("CLASS_FIELDS", &table.data.make_class_field());
        ctx.insert("enumerate_fields", &table.data.make_enum_field());
        ctx
    }
    fn make_manager(&self, table: &TableMerged) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("config", &self);

        ctx.insert(
            "tables",
            &table
                .table_names()
                .into_iter()
                .map(|name| {
                    let name = format!("{}{}", name, self.suffix_table);
                    let private = format!("_{}", name.to_case(Case::Snake));
                    CSharpTable { name, private }
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

#[derive(Serialize)]
struct CSharpEnum {
    name: String,
    number: String,
}
#[derive(Serialize)]
struct CSharpTable {
    name: String,
    private: String,
}

impl XData {
    fn make_class_field(&self) -> Vec<CSharpField> {
        match self {
            XData::Dictionary(v) => v.headers.iter().map(|v| v.make_class_field()).collect(),
            XData::Enumerate(v) => v.headers.iter().map(|v| v.make_class_field()).collect(),
        }
    }
    fn make_enum_field(&self) -> Vec<CSharpEnum> {
        match self {
            XData::Dictionary(_) => vec![],
            XData::Enumerate(v) => {
                v.data.values().map(|v| CSharpEnum { name: v.name.to_string(), number: v.id.to_string() }).collect()
            }
        }
    }
}

impl XCellHeader {
    fn make_class_field(&self) -> CSharpField {
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
