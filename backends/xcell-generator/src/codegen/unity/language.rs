use super::*;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use xcell_analyzer::WorkspaceManager;
use xcell_config::UnityCodegen;
use xcell_core::XResult;

/// Unity 语言表代码生成模板数据
pub struct UnityLanguageTemplate {
    /// 编译器版本
    pub compiler_version: &'static str,
    /// 二进制文件路径
    pub binary_path: String,
    /// Unity 代码生成配置
    pub config: UnityCodegen,
    /// 语言字段列表
    pub language_fields: Vec<LanguageField>,
}

/// 语言字段信息
#[derive(Clone, Debug)]
pub struct LanguageField {
    /// 语言类名
    pub class_name: String,
    /// 公开访问名称
    pub public_name: String,
    /// 私有存储名称
    pub private_name: String,
}

impl Display for LanguageField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl UnityGenerator {
    /// 写入 Unity 语言表代码
    ///
    /// # 参数
    /// * `ws` - 工作区管理器
    ///
    /// # 返回值
    /// 操作结果
    pub(super) fn write_language(&self, ws: &WorkspaceManager) -> XResult<()> {
        let root = &ws.config.root;
        let output_dir = self.config.loader_path(root);

        if let Some(parent) = output_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let path = output_dir.join("LanguageTable.cs");
        let mut file = std::fs::File::create(path)?;
        let template = make_language(&self.config, ws);
        let out = render_language(&template)?;
        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

/// 创建 Unity 语言表模板数据
fn make_language(config: &UnityCodegen, ws: &WorkspaceManager) -> UnityLanguageTemplate {
    let language_fields: Vec<LanguageField> = ws
        .languages()
        .iter()
        .map(|lang| LanguageField {
            class_name: format!("Language{}", lang.key),
            public_name: format!("language_{}", lang.key.to_case(Case::Snake)),
            private_name: format!("_language_{}", lang.key.to_case(Case::Snake)),
        })
        .collect();

    UnityLanguageTemplate {
        compiler_version: env!("CARGO_PKG_VERSION"),
        binary_path: String::new(),
        config: config.clone(),
        language_fields,
    }
}

/// 渲染 Unity 语言表代码
fn render_language(template: &UnityLanguageTemplate) -> XResult<String> {
    let mut sb = String::new();
    sb.push_str(&format!("// 代码生成, 修改无效! (XCell {})\n", template.compiler_version));
    sb.push_str("#pragma warning disable\n\n");
    sb.push_str("using System;\nusing System.Collections.Generic;\nusing System.IO;\nusing System.Runtime.Serialization;\nusing UnityEngine;\n\n");
    sb.push_str(&format!("namespace {}\n{{\n", template.config.namespace));
    sb.push_str("    [DataContract, Serializable]\n");
    sb.push_str("    public partial class LanguageTable\n    {\n");

    for field in &template.language_fields {
        sb.push_str(&format!("        [DataMember]\n"));
        sb.push_str(&format!("        private Dictionary<string, string> {} = new();\n", field.private_name));
        sb.push_str(&format!("        public Dictionary<string, string> {} => {};\n\n", field.public_name, field.private_name));
    }

    sb.push_str("\n        public void BinaryRead(string path)\n        {\n");
    sb.push_str("            using var stream = new FileStream(path, FileMode.Open, FileAccess.Read);\n");
    sb.push_str("            using var reader = new BinaryReader(stream);\n");
    sb.push_str("            BinaryRead(reader);\n");
    sb.push_str("        }\n\n");

    sb.push_str("        public void BinaryRead(BinaryReader r)\n        {\n");
    for field in &template.language_fields {
        sb.push_str(&format!("            {}.Clear();\n", field.private_name));
        sb.push_str(&format!("            {{\n                var count = r.ReadUInt32();\n                for (var i = 0; i < count; i++)\n                {{\n                    var key = r.ReadString();\n                    var value = r.ReadString();\n                    {}.Add(key, value);\n                }}\n            }}\n", field.private_name));
    }
    sb.push_str("        }\n\n");

    sb.push_str("        public void BinaryWrite(BinaryWriter w)\n        {\n");
    for field in &template.language_fields {
        sb.push_str(&format!("            {{\n                w.Write((uint){}.Count);\n                foreach (var kv in {})\n                {{\n                    w.Write(kv.Key);\n                    w.Write(kv.Value);\n                }}\n            }}\n", field.private_name, field.private_name));
    }
    sb.push_str("        }\n");

    sb.push_str("    }\n}\n");
    Ok(sb)
}
