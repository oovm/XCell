use super::*;

#[derive(Template)]
#[template(path = "BuildEnumerate.cs", ext = "txt", escape = "none")]
pub struct UnityEnumerate {
    compiler_version: &'static str,
    class_name: String,
    id_type: &'static str,
    config: UnityCodegen,
    enumerate_ids: Vec<EnumeratePair>,
    enumerate_fields: Vec<EnumerateField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumerateField {
    document: Vec<String>,
    switch: Vec<EnumeratePair>,
    name: String,
    typing: String,
    getter: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumeratePair {
    key: String,
    value: String,
    document: Vec<String>,
}

impl Display for EnumerateField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityCodegen {
    pub(super) fn write_enumerate(&self, ws: &WorkspaceManager, table: &XEnumerateData) -> XResult<()> {
        let out = match self.make_enumerate(table).render() {
            Ok(o) => o,
            Err(e) => Err(XError::runtime_error(format!("生成枚举失败: {}", e)))?,
        };
        let mut file = self.log_csharp(ws, &table.name)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
    fn make_enumerate(&self, table: &XEnumerateData) -> UnityEnumerate {
        UnityEnumerate {
            compiler_version: env!("CARGO_PKG_VERSION"),
            config: self.clone(),
            class_name: table.name.clone(),
            id_type: table.typing.kind.as_csharp_type(),
            enumerate_ids: table.lines.iter().map(|data| data.as_enumerate()).collect(),
            enumerate_fields: table.headers.iter().enumerate().map(|(id, data)| data.as_enumerate(&table.lines, id)).collect(),
        }
    }
}

impl XCellHeader {
    fn as_enumerate(&self, values: &[XDataLine], index: usize) -> EnumerateField {
        EnumerateField {
            name: self.field_name.clone(),
            typing: self.typing.as_csharp_type(),
            getter: format!("Get{}", self.field_name.to_case(Case::Pascal)),
            document: self.document.lines(),
            switch: values.iter().map(|data| data.as_pair(index)).collect(),
        }
    }
}

impl XDataLine {
    fn as_enumerate(&self) -> EnumeratePair {
        EnumeratePair { key: self.key.clone(), value: self.id.to_string(), document: self.comment.lines() }
    }
    fn as_pair(&self, index: usize) -> EnumeratePair {
        // 枚举和字段一样长, 必定存在
        let data = self.data.get(index).unwrap();
        EnumeratePair { key: self.key.clone(), value: data.as_csharp_value(), document: self.comment.lines() }
    }
}
