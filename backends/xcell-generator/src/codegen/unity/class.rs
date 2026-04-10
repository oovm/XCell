use super::*;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use xcell_analyzer::{WorkspaceManager, XClassData, XClassItem};
use xcell_config::UnityCodegen;
use xcell_core::{
    XResult,
    codegen::{CSharpReader, CSharpWriter},
};

/// Unity 类代码生成模板数据
pub struct UnityClassTemplate {
    /// 编译器版本
    pub compiler_version: &'static str,
    /// 类名
    pub class_name: String,
    /// 表名
    pub table_name: String,
    /// ID 类型
    pub id_type: &'static str,
    /// Unity 代码生成配置
    pub config: UnityCodegen,
    /// 键名
    pub key_name: String,
    /// 类字段列表
    pub class_fields: Vec<ClassField>,
}

/// 类字段信息
#[derive(Clone, Debug)]
pub struct ClassField {
    /// 字段文档
    pub document: Vec<String>,
    /// 字段名
    pub name: String,
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

/// 字段读取器信息
#[derive(Clone, Debug)]
pub struct FieldReader {
    /// 字段名
    pub field: String,
    /// 转换类型
    pub cast: String,
    /// 是否为向量
    pub is_vector: bool,
    /// 属性列表
    pub properties: Vec<String>,
}

/// 字段写入器信息
#[derive(Clone, Debug)]
pub struct FieldWriter {
    /// 字段名
    pub field: String,
    /// 转换类型
    pub cast: String,
    /// 是否为向量
    pub is_vector: bool,
    /// 属性列表
    pub properties: Vec<String>,
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

impl Display for ClassField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityGenerator {
    /// 写入 Unity 类代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    /// * `table` - 类数据表
    ///
    /// # 返回值
    /// 操作结果
    pub(super) fn write_class(&self, ws: &WorkspaceManager, table: &XClassData) -> XResult<()> {
        let root = &ws.config.root;
        let table_name = format!("{}{}", table.name, self.config.suffix_table);
        let output_dir = self.config.loader_path(root);

        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let path = output_dir.join(format!("{}.cs", table_name));
        let mut file = std::fs::File::create(path)?;
        let template = make_class(&self.config, table, table_name);
        let out = render_class(&template)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

/// 创建 Unity 类模板数据
///
/// # 参数
/// * `config` - Unity 代码生成配置
/// * `table` - 类数据表
/// * `table_name` - 表名
///
/// # 返回值
/// Unity 类模板数据
fn make_class(config: &UnityCodegen, table: &XClassData, table_name: String) -> UnityClassTemplate {
    UnityClassTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        config: config.clone(),
        table_name,
        class_name: table.name.clone(),
        key_name: "key".to_string(),
        id_type: "string",
        class_fields: table.items.iter().map(class_item_to_field).collect(),
    }
}

/// 将 XClassItem 转换为 ClassField
///
/// # 参数
/// * `item` - 类数据项
///
/// # 返回值
/// ClassField 表示
fn class_item_to_field(item: &XClassItem) -> ClassField {
    let default = item.typing.as_csharp_default();
    ClassField {
        document: item.document.lines(),
        name: item.field.clone(),
        typing: item.typing.as_csharp_type(),
        has_default: !default.is_empty(),
        default,
        getter: "<getter>".to_string(),
        reader: item.typing.make_cs_binary_reader(&item.field),
        writer: item.typing.make_cs_binary_writer(&item.field),
    }
}

/// 渲染 Unity 类代码
///
/// # 参数
/// * `template` - 类模板数据
///
/// # 返回值
/// 渲染后的 C# 代码字符串
fn render_class(template: &UnityClassTemplate) -> XResult<String> {
    let mut sb = String::new();
    sb.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n", template.compiler_version));
    sb.push_str("#pragma warning disable\n\n");
    sb.push_str(&format!("namespace {}\n{{\n", template.config.namespace));
    sb.push_str(&format!("    [System.Serializable]\n"));
    sb.push_str(&format!("    public partial class {}\n    {{\n", template.table_name));

    for field in &template.class_fields {
        for doc_line in &field.document {
            sb.push_str(&format!("        /// {}\n", doc_line));
        }
        if field.has_default {
            sb.push_str(&format!("        public {} {} = {};\n", field.typing, field.name, field.default));
        } else {
            sb.push_str(&format!("        public {} {};\n", field.typing, field.name));
        }
    }

    sb.push_str("\n        public void BinaryRead(System.IO.BinaryReader r)\n        {\n");
    for field in &template.class_fields {
        if field.reader.is_vector {
            sb.push_str(&format!("            {} = r.ReadList(r, {} => {});\n", field.name, field.reader.field, field.reader.function));
        } else {
            sb.push_str(&format!("            {} = {};\n", field.reader.field, field.reader.function));
        }
    }
    sb.push_str("        }\n");

    sb.push_str("\n        public void BinaryWrite(System.IO.BinaryWriter w)\n        {\n");
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
