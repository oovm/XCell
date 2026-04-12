use super::*;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use xcell_analyzer::{WorkspaceManager, XCellHeader, XDataLine, XEnumerateData};
use xcell_config::UnityCodegen;
use xcell_core::XResult;
use crate::codegen::core::csharp_ffi::{AsCSharpType, AsCSharpValue};

/// Unity 枚举代码生成模板数据
pub struct UnityEnumerateTemplate {
    /// 编译器版本
    pub compiler_version: &'static str,
    /// 类名
    pub class_name: String,
    /// ID 类型
    pub id_type: String,
    /// Unity 代码生成配置
    pub config: UnityCodegen,
    /// 枚举 ID 列表
    pub enumerate_ids: Vec<EnumeratePair>,
    /// 枚举字段列表
    pub enumerate_fields: Vec<EnumerateField>,
}

/// 枚举字段信息
#[derive(Clone, Debug)]
pub struct EnumerateField {
    /// 字段文档
    pub document: Vec<String>,
    /// Switch 分支
    pub switch: Vec<EnumeratePair>,
    /// 字段名
    pub name: String,
    /// 字段类型
    pub typing: String,
    /// Getter 方法
    pub getter: String,
}

/// 枚举键值对信息
#[derive(Clone, Debug)]
pub struct EnumeratePair {
    /// 键名
    pub key: String,
    /// 值
    pub value: String,
    /// 文档说明
    pub document: Vec<String>,
}

impl Display for EnumerateField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityGenerator {
    /// 写入 Unity 枚举代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `table` - 枚举数据表
    ///
    /// # 返回值
    /// 操作结果
    pub(super) fn write_enumerate(&self, ws: &WorkspaceManager, table: &XEnumerateData) -> XResult<()> {
        let root = &ws.config.root;
        let output_dir = self.config.loader_path(root);

        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let path = output_dir.join(format!("{}.cs", table.name));
        let mut file = std::fs::File::create(path)?;
        let template = make_enumerate(&self.config, table);
        let out = render_enumerate(&template)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

/// 创建 Unity 枚举模板数据
///
/// # 参数
/// * `config` - Unity 代码生成配置
/// * `table` - 枚举数据表
///
/// # 返回值
/// Unity 枚举模板数据
fn make_enumerate(config: &UnityCodegen, table: &XEnumerateData) -> UnityEnumerateTemplate {
    UnityEnumerateTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        config: config.clone(),
        class_name: table.name.clone(),
        id_type: table.typing.kind.as_csharp_type(),
        enumerate_ids: table.lines.iter().map(data_line_to_id_pair).collect(),
        enumerate_fields: table.headers.iter().enumerate().map(|(id, data)| header_to_enumerate_field(data, &table.lines, id)).collect(),
    }
}

/// 将 XDataLine 转换为 ID 枚举对
///
/// # 参数
/// * `data` - 数据行
///
/// # 返回值
/// EnumeratePair 表示
fn data_line_to_id_pair(data: &XDataLine) -> EnumeratePair {
    EnumeratePair {
        key: data.key.clone(),
        value: data.id.to_string(),
        document: data.comment.lines(),
    }
}

/// 将 XCellHeader 转换为 EnumerateField
///
/// # 参数
/// * `header` - 表头
/// * `values` - 数据行
/// * `index` - 字段索引
///
/// # 返回值
/// EnumerateField 表示
fn header_to_enumerate_field(header: &XCellHeader, values: &[XDataLine], index: usize) -> EnumerateField {
    EnumerateField {
        name: header.field_name.clone(),
        typing: header.typing.as_csharp_type(),
        getter: format!("Get{}", header.field_name.to_case(Case::Pascal)),
        document: header.document.lines(),
        switch: values.iter().map(|data| data_line_to_value_pair(data, index)).collect(),
    }
}

/// 将 XDataLine 转换为字段值枚举对
///
/// # 参数
/// * `data` - 数据行
/// * `index` - 字段索引
///
/// # 返回值
/// EnumeratePair 表示
fn data_line_to_value_pair(data: &XDataLine, index: usize) -> EnumeratePair {
    let value = data.data.get(index).unwrap();
    EnumeratePair {
        key: data.key.clone(),
        value: value.as_csharp_value(),
        document: data.comment.lines(),
    }
}

/// 渲染 Unity 枚举代码
///
/// # 参数
/// * `template` - 枚举模板数据
///
/// # 返回值
/// 渲染后的 C# 代码字符串
fn render_enumerate(template: &UnityEnumerateTemplate) -> XResult<String> {
    let mut sb = String::new();
    sb.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n", template.compiler_version));
    sb.push_str("#pragma warning disable\n\n");
    sb.push_str(&format!("namespace {}\n{{\n", template.config.namespace));

    sb.push_str(&format!("    public enum {}ID : {}\n    {{\n", template.class_name, template.id_type));
    for pair in &template.enumerate_ids {
        for doc_line in &pair.document {
            if !doc_line.is_empty() {
                sb.push_str(&format!("        /// {}\n", doc_line));
            }
        }
        sb.push_str(&format!("        {} = {},\n", pair.key, pair.value));
    }
    sb.push_str("    }\n\n");

    sb.push_str(&format!("    public static class {}Extensions\n    {{\n", template.class_name));
    for field in &template.enumerate_fields {
        for doc_line in &field.document {
            if !doc_line.is_empty() {
                sb.push_str(&format!("        /// {}\n", doc_line));
            }
        }
        sb.push_str(&format!("        public static {} {}(this {}ID id)\n        {{\n", field.typing, field.getter, template.class_name));
        sb.push_str("            switch (id)\n            {\n");
        for pair in &field.switch {
            sb.push_str(&format!("                case {}ID.{}: return {};\n", template.class_name, pair.key, pair.value));
        }
        sb.push_str("                default: return default;\n");
        sb.push_str("            }\n");
        sb.push_str("        }\n\n");
    }
    sb.push_str("    }\n}\n");
    Ok(sb)
}
