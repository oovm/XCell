use super::*;

#[derive(Template)]
#[template(path = "BuildDictionary.cs.djv", ext = "txt", escape = "none")]
pub struct UnityDictionary {
    version: &'static str,
    class_name: String,
    public_name: String,
    table_name: String,
    id_type: &'static str,
    config: UnityCodegen,
    key_name: String,
    enumerate: String,
    enumerate_ids: Vec<EnumeratePair>,
    class_fields: Vec<DictField>,
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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictField {
    number: String,
    document: Vec<String>,
    remarks: Vec<EnumeratePair>,
    name: String,
    typing: String,
    getter: String,
    has_default: bool,
    default: String,
    reader: CSharpReader,
    writer: CSharpWriter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldReader {
    field: String,
    cast: String,
    is_vector: bool,
    properties: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldWriter {
    field: String,
    cast: String,
    is_vector: bool,
    properties: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    key: String,
    value: String,
    document: Vec<String>,
}

impl Display for DictField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        let out = match self.make_dict(table).render() {
            Ok(o) => o,
            Err(e) => Err(XError::runtime_error(format!("生成表单失败: {}", e)))?,
        };
        let mut file = self.log_csharp(ws, &table.name)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    fn make_dict(&self, table: &XDictData) -> UnityDictionary {
        UnityDictionary {
            version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            key_name: "<key_name>".to_string(),
            table_name: table.name.clone(),
            id_type: "<id_type>",
            enumerate_ids: vec![],
            class_fields: vec![DictField {
                number: "<number>".to_string(),
                document: vec![],
                remarks: vec![],
                name: "<name>".to_string(),
                typing: "<typing>".to_string(),
                getter: "<getter>".to_string(),
                has_default: true,
                default: "<default>".to_string(),
                reader: CSharpReader { is_vector: false, field: "<field>".to_string(), function: "<function>".to_string() },
                writer: CSharpWriter {
                    is_vector: false,
                    field: "<field>".to_string(),
                    cast: "<cast>".to_string(),
                    properties: vec![],
                },
            }],
            public_name: "<public_name>".to_string(),
            enumerate: "<enumerate>".to_string(),
            class_name: "<class_name>".to_string(),
        }
    }
}

impl XCellHeader {
    fn as_dict(&self, values: &[XDataLine], index: usize) -> DictField {
        DictField {
            number: "<number>".to_string(),
            name: self.field_name.clone(),
            typing: self.typing.as_csharp_type(),
            getter: format!("Get{}", self.field_name.to_case(Case::Pascal)),
            has_default: true,
            document: self.comment.lines(),
            remarks: values.iter().map(|data| data.as_pair2(index)).collect(),
            default: "<default>".to_string(),
            reader: CSharpReader { is_vector: false, field: "".to_string(), function: "".to_string() },
            writer: CSharpWriter { is_vector: false, field: "".to_string(), cast: "".to_string(), properties: vec![] },
        }
    }
}

impl XDataLine {
    fn as_enumerate2(&self) -> EnumeratePair {
        EnumeratePair { key: self.key.clone(), value: self.id.to_string(), document: self.comment.lines() }
    }
    fn as_pair2(&self, index: usize) -> EnumeratePair {
        // 枚举和字段一样长, 必定存在
        let data = self.data.get(index).unwrap();
        EnumeratePair { key: self.key.clone(), value: data.as_csharp_value(), document: self.comment.lines() }
    }
}
