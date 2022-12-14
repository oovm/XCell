use crate::XClassItem;

use super::*;

#[derive(Serialize)]
pub struct UnityManagerWriter {
    compiler_version: &'static str,
    table_version: String,
    edit_time: String,
    config: UnityCodegen,
    tables: Vec<String>,
}

impl UnityManagerWriter {
    pub fn new(table: &MergedTable, unity: &UnityCodegen, version: &str) -> Self {
        Self {
            compiler_version: env!("CARGO_PKG_VERSION"),
            table_version: version.to_string(),
            edit_time: XCellValueKind::csharp_now(),
            config: unity.clone(),
            tables: table.table_names(&unity.suffix_table).into_iter().sorted().collect(),
        }
    }
}

impl UnityCodegen {
    pub fn ensure_path(&self, root: &Path) -> XResult<()> {
        if let Some(s) = self.unity_csharp_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        if let Some(s) = self.unity_binary_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        if let Some(s) = self.unity_xml_path(root, "test")?.parent() {
            create_dir_all(s)?
        }
        Ok(())
    }
    pub fn write_manager(&self, table: &MergedTable, root: &Path, version: &str) -> XResult<()> {
        let path = self.unity_manager_path(root)?;
        let ctx = Context::from_serialize(UnityManagerWriter::new(table, self, version))?;
        tera_render(include_str!("PartManager.cs.djv"), &ctx, &path, "PartManager.cs")?;
        Ok(())
    }
    pub fn write_class(&self, table: &XTable, root: &Path) -> XResult<()> {
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = self.unity_csharp_path(root, &file)?;
        log::info!("写入 {}", self.unity_cs_relative(&file));
        match table.data {
            XTableKind::Array(_) => {
                tera_render(include_str!("PartDictionary.cs.djv"), &self.make_context(table), &path, "PartClass.cs")?;
            }
            XTableKind::Enumerate(_) => {
                tera_render(include_str!("PartDictionary.cs.djv"), &self.make_context(table), &path, "PartClass.cs")?;
            }
            XTableKind::Class(_) => {
                tera_render(include_str!("PartClass.cs.djv"), &self.make_context(table), &path, "PartClass.cs")?;
            }
            XTableKind::Dictionary(_) => {
                todo!()
            }
        }
        Ok(())
    }
    pub fn write_binary(&self, table: &XTable, root: &Path) -> XResult<()> {
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = self.unity_binary_path(root, &file)?;
        log::info!("写入 {}", self.unity_bin_relative(&file));
        let cg = BinaryWriter::default();
        cg.write_binary(table, &path)
    }
    pub fn write_data_contract(&self, table: &XTable, root: &Path) -> XResult<()> {
        if !self.xml.enable {
            return Ok(());
        }
        let file = format!("{}{}", table.name, self.suffix_table);
        let path = self.unity_xml_path(root, &file)?;
        log::info!("写入 {}", self.unity_xml_relative(&file));
        let cg = DataContractWriter::new(&self.namespace, table, &self.suffix_table);
        cg.write_xml(&path)
    }
}

impl UnityCodegen {
    fn make_context(&self, table: &XTable) -> Context {
        let mut ctx = Context::new();
        ctx.insert("VERSION", env!("CARGO_PKG_VERSION"));
        ctx.insert("config", &self);
        ctx.insert("CLASS_NAME", &table.name);
        ctx.insert("TABLE_NAME", &format!("{}{}", table.name, self.suffix_table));
        ctx.insert("KEY", &table.data.key_field());
        ctx.insert("ID_TYPE", &table.data.key_type().as_csharp_type());
        let is_enum = table.is_enumerate();
        ctx.insert("enumerate", &is_enum);
        ctx.insert("CLASS_FIELDS", &table.data.make_class_field());
        ctx.insert("enumerate_fields", &table.data.make_enum_field());
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

impl XTableKind {
    fn make_class_field(&self) -> Vec<CSharpField> {
        match self {
            XTableKind::Array(v) => v.headers.iter().map(|v| v.make_class_field()).collect(),
            XTableKind::Enumerate(v) => v.headers.iter().map(|v| v.make_class_field()).collect(),
            XTableKind::Class(v) => v.items.iter().map(|v| v.make_class_field()).collect(),
            XTableKind::Dictionary(_) => {
                todo!()
            }
        }
    }
    fn make_enum_field(&self) -> Vec<CSharpEnum> {
        match self {
            XTableKind::Array(_) => vec![],
            XTableKind::Enumerate(v) => {
                v.data.values().map(|v| CSharpEnum { name: v.name.to_string(), number: v.id.to_string() }).collect()
            }
            XTableKind::Class(_) => {
                vec![]
            }
            XTableKind::Dictionary(_) => {
                todo!()
            }
        }
    }
}

impl XClassItem {
    fn make_class_field(&self) -> CSharpField {
        let default = self.r#type.as_csharp_default();
        CSharpField {
            summary: self.summary.lines().map(|v| v.to_string()).collect(),
            remarks: self.details.lines().map(|v| v.to_string()).collect(),
            has_default: !default.is_empty(),
            typing: self.r#type.as_csharp_type(),
            writer: self.r#type.make_cs_binary_writer(&self.field),
            reader: self.r#type.make_cs_binary_reader(&self.field),
            name: self.field.clone(),
            getter: format!("Get{}", self.field.to_case(Case::Pascal)),
            default,
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
