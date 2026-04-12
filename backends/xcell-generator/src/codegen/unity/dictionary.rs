use super::*;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use xcell_analyzer::{WorkspaceManager, XCellHeader, XDictData, XListData};
use xcell_config::UnityCodegen;
use xcell_provider::XCellAccess;
use xcell_core::XResult;
use crate::codegen::core::csharp_ffi::{
    AsCSharpType, AsCSharpDefault, CSharpBinaryReader, CSharpBinaryWriter, CSharpReader, CSharpWriter,
};

/// Unity 字典代码生成模板数据
pub struct UnityDictionaryTemplate {
    /// 编译器版本
    pub compiler_version: &'static str,
    /// 类名
    pub class_name: String,
    /// 表名
    pub table_name: String,
    /// ID 类型
    pub id_type: String,
    /// Unity 代码生成配置
    pub config: UnityCodegen,
    /// 键名
    pub key_name: String,
    /// 字典字段列表
    pub class_fields: Vec<DictField>,
}

/// 字典字段信息
#[derive(Clone, Debug)]
pub struct DictField {
    /// 字段文档
    pub document: Vec<String>,
    /// 字段名
    pub name: String,
    /// 访问修饰符
    pub access: &'static str,
    /// 字段类型
    pub typing: String,
    /// Getter 方法
    pub getter: String,
    /// 是否有默认值
    pub has_default: bool,
    /// 默认值
    pub default: String,
    /// C# 读取器代码
    pub reader: CSharpReader,
    /// C# 写入器代码
    pub writer: CSharpWriter,
}

impl Display for DictField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityGenerator {
    /// 写入 Unity 字典代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `table` - 字典数据表
    ///
    /// # 返回值
    /// 操作结果
    pub(super) fn write_dict(&self, ws: &WorkspaceManager, table: &XDictData) -> XResult<()> {
        let root = &ws.config.root;
        let table_name = format!("{}{}", table.name, self.config.suffix_table);
        let output_dir = self.config.loader_path(root);

        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let path = output_dir.join(format!("{}.cs", table_name));
        let mut file = std::fs::File::create(path)?;
        let template = make_dict(&self.config, table, table_name);
        let out = render_dictionary(&template)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }

    /// 写入 Unity 列表代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `table` - 列表数据表
    ///
    /// # 返回值
    /// 操作结果
    pub(super) fn write_list(&self, ws: &WorkspaceManager, table: &XListData) -> XResult<()> {
        let root = &ws.config.root;
        let table_name = format!("{}{}", table.name, self.config.suffix_table);
        let output_dir = self.config.loader_path(root);

        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let path = output_dir.join(format!("{}.cs", table_name));
        let mut file = std::fs::File::create(path)?;
        let template = make_list(&self.config, table, table_name);
        let out = render_dictionary(&template)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

/// 创建 Unity 字典模板数据
fn make_dict(config: &UnityCodegen, table: &XDictData, table_name: String) -> UnityDictionaryTemplate {
    UnityDictionaryTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        config: config.clone(),
        table_name,
        class_name: table.name.clone(),
        key_name: "key".to_string(),
        id_type: "string".to_string(),
        class_fields: table.headers.iter().map(header_to_dict_field).collect(),
    }
}

/// 创建 Unity 列表模板数据
fn make_list(config: &UnityCodegen, table: &XListData, table_name: String) -> UnityDictionaryTemplate {
    UnityDictionaryTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        config: config.clone(),
        table_name,
        class_name: table.name.clone(),
        key_name: "id".to_string(),
        id_type: table.id_type.as_csharp_type(),
        class_fields: table.headers.iter().map(header_to_dict_field).collect(),
    }
}

/// 将 XCellHeader 转换为 DictField
fn header_to_dict_field(header: &XCellHeader) -> DictField {
    let default = header.typing.as_csharp_default();
    let access = match header.access {
        XCellAccess::Default => "",
        XCellAccess::Public => "public ",
        XCellAccess::Private => "private ",
    };
    DictField {
        document: header.document.lines(),
        name: header.field_name.clone(),
        access,
        typing: header.typing.as_csharp_type(),
        has_default: !default.is_empty(),
        default,
        getter: "<getter>".to_string(),
        reader: header.typing.make_cs_binary_reader(&header.field_name),
        writer: header.typing.make_cs_binary_writer(&header.field_name),
    }
}

/// 渲染 Unity 字典/列表代码
fn render_dictionary(template: &UnityDictionaryTemplate) -> XResult<String> {
    let mut sb = String::new();
    sb.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n", template.compiler_version));
    sb.push_str("#pragma warning disable\n\n");
    sb.push_str("using System;\nusing System.Collections.Generic;\nusing System.IO;\n\n");
    sb.push_str(&format!("namespace {}\n{{\n", template.config.namespace));
    sb.push_str(&format!("    [System.Serializable]\n"));
    sb.push_str(&format!("    public partial class {} : IReadOnlyDictionary<{}, {}>\n    {{\n", template.table_name, template.id_type, template.class_name));

    for field in &template.class_fields {
        for doc_line in &field.document {
            if !doc_line.is_empty() {
                sb.push_str(&format!("        /// {}\n", doc_line));
            }
        }
        if field.has_default {
            sb.push_str(&format!("        {}{} {} = {};\n", field.access, field.typing, field.name, field.default));
        } else {
            sb.push_str(&format!("        {}{} {};\n", field.access, field.typing, field.name));
        }
    }

    sb.push_str("\n        public void BinaryRead(BinaryReader r)\n        {\n");
    for field in &template.class_fields {
        if field.reader.is_vector {
            sb.push_str(&format!("            {} = r.ReadList(r, {} => {});\n", field.name, field.reader.field, field.reader.function));
        } else {
            sb.push_str(&format!("            {} = {};\n", field.reader.field, field.reader.function));
        }
    }
    sb.push_str("        }\n");

    sb.push_str("\n        public void BinaryWrite(BinaryWriter w)\n        {\n");
    for field in &template.class_fields {
        if field.writer.is_vector {
            sb.push_str(&format!("            w.WriteList({}, {});\n", field.writer.field, field.writer.cast));
        } else {
            sb.push_str(&format!("            w.Write({});\n", field.writer.cast));
        }
    }
    sb.push_str("        }\n");

    sb.push_str("    }\n}\n");
    Ok(sb)
}
